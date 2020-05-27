use crate::sinks::Sink;
use crate::Message;
use log::debug;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

struct Channels<T: std::clone::Clone> {
    input_rx: Option<Receiver<Message<T>>>,
    input_txs: Vec<Sender<Message<T>>>,
}

pub struct DataSet<T: std::clone::Clone> {
    channels: Arc<Mutex<Channels<T>>>,
    threads: Vec<Option<thread::JoinHandle<()>>>,
    registry: Arc<Mutex<Vec<Sender<()>>>>,
    registered: bool,
    name: String,
}

impl<T: std::clone::Clone + std::marker::Send + 'static> DataSet<T> {
    pub fn new(
        input_rx: Receiver<Message<T>>,
        registry: Arc<Mutex<Vec<Sender<()>>>>,
    ) -> DataSet<T> {
        let channels = Arc::new(Mutex::new(Channels {
            input_rx: Some(input_rx),
            input_txs: Vec::new(),
        }));
        DataSet {
            channels,
            threads: Vec::new(),
            registry,
            registered: false,
            name: String::new(),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    pub fn map<U: 'static, F: 'static>(&mut self, f: F) -> DataSet<U>
    where
        F: std::marker::Sync + std::marker::Send + Fn(T) -> U,
        Self: std::marker::Sized,
        U: std::clone::Clone + std::marker::Send,
    {
        let (input_tx, input_rx) = mpsc::channel::<Message<T>>();
        let (output_tx, output_rx) = mpsc::channel::<Message<U>>();

        let thread = thread::spawn(move || loop {
            let input = input_rx.recv().unwrap();
            match input {
                Message::Data(data) => {
                    let output = f(data);
                    if output_tx.send(Message::Data(output)).is_err() {
                        break;
                    }
                }
                Message::Terminate => {
                    output_tx.send(Message::Terminate).unwrap();
                    break;
                }
            }
        });

        self.channels.lock().unwrap().input_txs.push(input_tx);
        debug!("Pushing map thread");
        self.threads.push(Some(thread));

        if !self.registered {
            self.register();
        }

        if self.name.is_empty() {
            self.name = "Map Processor".to_string();
        }

        DataSet::new(output_rx, Arc::clone(&self.registry))
    }

    pub fn filter<F: 'static>(&mut self, f: F) -> DataSet<T>
    where
        F: std::marker::Send + Fn(&T) -> bool,
        Self: std::marker::Sized,
    {
        let (input_tx, input_rx) = mpsc::channel::<Message<T>>();
        let (output_tx, output_rx) = mpsc::channel::<Message<T>>();
        let thread = thread::spawn(move || {
            loop {
                // receive data from input channel.
                let input = input_rx.recv().unwrap();
                match input {
                    Message::Data(data) => {
                        if f(&data) && output_tx.send(Message::Data(data)).is_err() {
                            break;
                        }
                    }
                    Message::Terminate => {
                        output_tx.send(Message::Terminate).unwrap();
                        break;
                    }
                }
            }
        });

        self.channels.lock().unwrap().input_txs.push(input_tx);
        debug!("Pushing filter thread");
        self.threads.push(Some(thread));

        if !self.registered {
            self.register();
        }

        if self.name.is_empty() {
            self.name = "Filter Processor".to_string();
        }

        DataSet::new(output_rx, Arc::clone(&self.registry))
    }

    pub fn add_sink<S: 'static>(&mut self, sink: S)
    where
        S: std::marker::Send + Sink<T = T>,
    {
        let (input_tx, input_rx) = mpsc::channel::<Message<T>>();
        // let input_rx = self.channels.lock().unwrap().input_rx.take().unwrap();
        let thread = thread::spawn(move || {
            sink.start(input_rx).expect("Error starting sink");
        });

        self.channels.lock().unwrap().input_txs.push(input_tx);
        debug!("Pushing sink thread");
        self.threads.push(Some(thread));

        if !self.registered {
            self.register();
        }
    }

    fn register(&mut self) {
        let (signal_tx, signal_rx) = mpsc::channel::<()>();
        let channels = Arc::clone(&self.channels);
        let thread = thread::spawn(move || {
            signal_rx.recv().unwrap();

            debug!("Received signal to setup and start processor.");

            // Do some plumbing.
            let input_rx = channels.lock().unwrap().input_rx.take().unwrap();
            let input_txs = &channels.lock().unwrap().input_txs;
            loop {
                let input = input_rx.recv().unwrap();
                match input {
                    Message::Data(data) => {
                        for input_tx in input_txs {
                            if input_tx.send(Message::Data(data.clone())).is_err() {
                                break;
                            }
                        }
                    }
                    Message::Terminate => {
                        for input_tx in input_txs {
                            input_tx.send(Message::Terminate).unwrap();
                        }
                        break;
                    }
                }
            }
        });
        debug!("Pushing register thread");
        self.threads.push(Some(thread));
        self.registry.lock().unwrap().push(signal_tx);
        self.registered = true;
    }
}

impl<T: std::clone::Clone> Drop for DataSet<T> {
    fn drop(&mut self) {
        debug!("Closing {}", self.name);
        for thread in &mut self.threads {
            debug!("Closing thread");
            if let Some(t) = thread.take() {
                t.join().unwrap();
            }
        }
    }
}

// TODO: Give option to set names to DataSet. Generate one if not set.

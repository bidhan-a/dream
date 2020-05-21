use crate::sinks::Sink;
use crate::Message;
use std::sync::mpsc::{self, Receiver};
use std::thread;

pub struct DataSet<T> {
    input_rx: Option<Receiver<Message<T>>>,
    thread: Option<thread::JoinHandle<()>>,
    has_sink: bool,
}

impl<T: std::marker::Send + 'static> DataSet<T> {
    pub fn new(input_rx: Receiver<Message<T>>) -> DataSet<T> {
        DataSet {
            input_rx: Some(input_rx),
            thread: None,
            has_sink: false,
        }
    }

    pub fn map<U: 'static, F: 'static>(&mut self, f: F) -> DataSet<U>
    where
        F: std::marker::Sync + std::marker::Send + Fn(T) -> U,
        Self: std::marker::Sized,
        U: std::marker::Send,
    {
        let (output_tx, output_rx) = mpsc::channel::<Message<U>>();
        let input_rx = self.input_rx.take().unwrap();
        let thread = thread::spawn(move || {
            loop {
                // receive data from input channel.
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
            }
        });
        self.thread = Some(thread);
        DataSet::new(output_rx)
    }

    pub fn filter<F: 'static>(&mut self, f: F) -> DataSet<T>
    where
        F: std::marker::Send + Fn(&T) -> bool,
        Self: std::marker::Sized,
    {
        let (output_tx, output_rx) = mpsc::channel::<Message<T>>();
        let input_rx = self.input_rx.take().unwrap();
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
        self.thread = Some(thread);
        DataSet::new(output_rx)
    }

    pub fn add_sink<S: 'static>(&mut self, sink: S)
    where
        S: std::marker::Send + Sink<T = T>,
    {
        let input_rx = self.input_rx.take().unwrap();
        let thread = thread::spawn(move || {
            sink.start(input_rx).expect("Error starting sink");
        });
        self.thread = Some(thread);
        self.has_sink = true; // use this later
    }
}

impl<T> Drop for DataSet<T> {
    fn drop(&mut self) {
        if self.has_sink {
            println!("Closing sink.");
        } else {
            println!("Closing processor.");
        }
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

// TODO: Give option to set names to DataSet. Generate one if not set.

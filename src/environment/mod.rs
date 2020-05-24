use crate::dataset::DataSet;
use crate::sources::Source;
use crate::Message;
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct Environment {
    name: String,
    source_runners: Vec<Option<SourceRunner>>,
    source_threads: Vec<Option<thread::JoinHandle<()>>>,
    start_signals: Arc<Mutex<Vec<Sender<()>>>>,
}

struct SourceRunner(Box<dyn FnOnce() + std::marker::Send + 'static>);

impl Environment {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            source_runners: Vec::new(),
            source_threads: Vec::new(),
            start_signals: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_source<S: 'static>(&mut self, source: S) -> DataSet<S::T>
    where
        S: std::marker::Send + Source,
        <S as Source>::T: std::marker::Send,
    {
        let (source_tx, source_rx) = mpsc::channel::<Message<S::T>>();

        let x = SourceRunner(Box::new(move || {
            source.start(source_tx).expect("Error starting source");
        }));

        self.source_runners.push(Some(x));

        DataSet::new(source_rx, Arc::clone(&self.start_signals))
    }

    pub fn run(&mut self) {
        println!("Starting {}.", self.name);
        for source_runner in &mut self.source_runners {
            let runner = source_runner.take();
            let thread = thread::spawn(move || {
                runner.unwrap().0();
            });
            self.source_threads.push(Some(thread));
        }
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        println!("Closing execution environment.");
        for thread in &mut self.source_threads {
            if let Some(thread) = thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

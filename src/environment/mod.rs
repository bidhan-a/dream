use crate::dataset::DataSet;
use crate::flow::Flow;
use crate::sources::Source;
use crate::Message;
use log::debug;
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;

pub struct Environment {
    name: String,
    source_runners: Vec<Option<SourceRunner>>,
    source_threads: Vec<Option<thread::JoinHandle<()>>>,
    registry: Arc<Mutex<Vec<Sender<()>>>>,
}

struct SourceRunner(Box<dyn FnOnce() + std::marker::Send + 'static>);

impl Environment {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            source_runners: Vec::new(),
            source_threads: Vec::new(),
            registry: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_source<S: 'static>(&mut self, source: S) -> DataSet<S::T>
    where
        S: std::marker::Send + Source,
        <S as Source>::T: std::marker::Send,
        <S as Source>::T: std::clone::Clone,
    {
        let (source_tx, source_rx) = mpsc::channel::<Message<S::T>>();
        let name = source.name();

        let x = SourceRunner(Box::new(move || {
            source.start(source_tx).expect("Error starting source");
        }));

        self.source_runners.push(Some(x));

        DataSet::new(source_rx, Arc::clone(&self.registry))
            .name(format!("{} Processor", name).as_str())
    }

    pub fn run(&mut self) {
        debug!("Starting {}.", self.name);

        // Signal the DataSets to get ready.
        let mut registry = self.registry.lock().unwrap();
        // Start in reverse order to ensure that downstream receivers
        // have been set up properly.
        registry.reverse();
        for tx in registry.iter() {
            tx.send(()).unwrap();
            thread::sleep(time::Duration::from_millis(10));
        }

        // Start sources.
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
        debug!("Closing execution environment.");
        for thread in &mut self.source_threads {
            if let Some(thread) = thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

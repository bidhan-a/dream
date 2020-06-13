use crate::Stats;
use log::debug;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;

pub struct Processor {
    id: String,
    name: String,
    stats: Arc<Mutex<Stats>>,
    start_signal_tx: Option<Sender<()>>,
    stats_rx: Option<Receiver<Stats>>,
    stats_thread: Option<thread::JoinHandle<()>>,
}

impl Processor {
    pub fn new(
        id: String,
        name: String,
        start_signal_tx: Option<Sender<()>>,
        stats_rx: Receiver<Stats>,
    ) -> Self {
        Processor {
            id,
            name,
            stats: Arc::new(Mutex::new(Stats::new(0, 0, 0, 0))),
            start_signal_tx,
            stats_rx: Some(stats_rx),
            stats_thread: None,
        }
    }

    pub fn start(&mut self) {
        // Start the underlying Dataset.
        if self.start_signal_tx.is_some() {
            let start_signal_tx = self.start_signal_tx.take().unwrap();
            start_signal_tx.send(()).unwrap();
        }

        thread::sleep(time::Duration::from_millis(10));

        // Setup stats thread for this processor.
        let stats_rx = self.stats_rx.take().unwrap();
        let stats = Arc::clone(&self.stats);
        let thread = thread::spawn(move || loop {
            let st = stats_rx.recv().unwrap();
            let ret = stats.lock().unwrap().update(st);
            if !ret {
                break;
            }
        });
        self.stats_thread = Some(thread);
        debug!("Started Processor: {}", self.name);
    }
}

impl Drop for Processor {
    fn drop(&mut self) {
        let stats = self.stats.lock().unwrap();
        debug!(
            "Closing Processor with id {} and name [{}]",
            self.id, self.name
        );
        debug!(
            "Records In: {}, Records Out: {}, Bytes In: {}, Bytes Out: {}",
            stats.get_records_in(),
            stats.get_records_out(),
            stats.get_bytes_in(),
            stats.get_bytes_out()
        );
        debug!("-----------------------");
        if let Some(t) = self.stats_thread.take() {
            t.join().unwrap();
        }
    }
}

#[derive(Default)]
pub struct Flow {
    processors: Vec<Processor>,
    edges: Vec<(String, String)>,
}

impl Flow {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, processor: Processor) {
        self.processors.push(processor);
    }

    pub fn add_edge(&mut self, edge: (String, String)) {
        self.edges.push(edge);
    }

    pub fn start(&mut self) {
        // debug!("{:?}", self.edges);
        // Start in reverse order to ensure that downstream receivers
        // have been set up properly.
        self.processors.reverse();
        for p in &mut self.processors {
            p.start();
        }
    }
}

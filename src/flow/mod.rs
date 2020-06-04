use crate::Stats;
use std::sync::mpsc::{self, Receiver, Sender};
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
    pub fn new(id: String, name: String) -> Self {
        Processor {
            id,
            name,
            stats: Arc::new(Mutex::new(Stats::new())),
            start_signal_tx: None,
            stats_rx: None,
            stats_thread: None,
        }
    }

    pub fn start(&mut self) {
        // Start the underlying Dataset.
        let start_signal_tx = self.start_signal_tx.take().unwrap();
        start_signal_tx.send(()).unwrap();

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
    }
}

pub struct Flow {
    processors: Vec<Processor>,
    edges: Vec<(String, String)>,
}

impl Flow {
    pub fn add(&mut self, processor: Processor, incoming_processor_id: String) {
        let processor_id = processor.id.clone();
        self.processors.push(processor);
        self.edges.push((incoming_processor_id, processor_id));
    }

    pub fn start(&mut self) {
        // Start in reverse order to ensure that downstream receivers
        // have been set up properly.
        self.processors.reverse();
        for p in &mut self.processors {
            p.start();
            thread::sleep(time::Duration::from_millis(10));
        }
        // Go back to original order.
        self.processors.reverse();
    }
}

/*


Processor {
    id: String,
    name: String,
    start_signal: channel,
    records_in: int,
    records_out: int,
    bytes_in: int,
    bytes_out: int
}


*/

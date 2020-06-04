use crate::Stats;
use std::sync::mpsc::{self, Sender};

pub struct Processor {
    id: String,
    name: String,
    stats: Stats,
    start_signal_tx: Option<Sender<()>>,
}

impl Processor {
    pub fn new(id: String, name: String) -> Self {
        Processor {
            id,
            name,
            stats: Stats::new(),
            start_signal_tx: None,
        }
    }
}

pub struct Flow {
    processors: Vec<Processor>,
    edges: Vec<(String, String)>,
}

impl Flow {
    pub fn add(mut self, processor: Processor, incoming_processor_id: String) {
        let processor_id = processor.id.clone();
        self.processors.push(processor);
        self.edges.push((incoming_processor_id, processor_id));
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

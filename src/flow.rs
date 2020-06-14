use crate::processor::Processor;

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

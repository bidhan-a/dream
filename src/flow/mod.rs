use daggy::Dag;

pub struct Processor {
    id: String,
    name: String,
}

impl Processor {
    pub fn new(id: String, name: String) -> Self {
        Processor { id, name }
    }
}

pub struct Flow {
    processors: Vec<Processor>,
    edges: Vec<(String, String)>,
    dag: Dag<Processor, ()>,
}

impl Flow {
    pub fn add(mut self, processor: Processor) {
        self.dag.add_node(processor);
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

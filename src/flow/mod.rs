use daggy::Dag;

pub struct Processor {
    name: String,
    id: String,
}

pub struct Flow {
    dag: Dag<Processor, ()>,
}

impl Flow {}

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

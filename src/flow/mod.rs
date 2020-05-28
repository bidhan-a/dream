use daggy::Dag;

pub trait Processor {
    fn name(self) -> String;
}

pub struct Flow {
    dag: Dag<Box<dyn Processor>, ()>,
}

impl Flow {}

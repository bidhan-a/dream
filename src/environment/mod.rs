use crate::dataset::DataSet;
use crate::sources::Source;
use crate::Message;
use std::sync::mpsc;

pub struct Environment {
    source_runners: Vec<SourceRunner>,
}

struct SourceRunner(Box<dyn FnOnce() -> ()>);

impl Environment {
    pub fn add_source<S: 'static>(&mut self, source: S) -> DataSet<S::T>
    where
        S: Source,
        <S as Source>::T: std::marker::Send,
    {
        let (source_tx, source_rx) = mpsc::channel::<Message<S::T>>();

        let x = SourceRunner(Box::new(move || {
            source.start(source_tx).expect("Error starting source");
        }));

        self.source_runners.push(x);

        DataSet::new(source_rx)
    }
}

/*



// GOAL

let env = Environment::new("My pipeline");
DataSet<String> ds = env.add_source(source);

// TODO: OTHER PROCESSING STEPS (map, reduce, etc.)

ds.add_sink(sink);
env.run();



Dataset
- input channel
- output channel


*/

use crate::dataset::DS;
use crate::sinks::Sink;
use crate::sources::Source;
use std::sync::mpsc;

pub struct Environment {
    source_runners: Vec<SourceRunner>,
}

struct SourceRunner(Box<dyn FnOnce() -> ()>);

impl Environment {
    pub fn add_source<S: 'static>(&mut self, source: S) -> DS<S::T>
    where
        S: Source,
        <S as Source>::T: std::marker::Send,
    {
        let (source_tx, source_rx) = mpsc::channel::<S::T>();

        let x = SourceRunner(Box::new(move || {
            source.start(source_tx).expect("Error starting source");
        }));

        self.source_runners.push(x);

        DS::new(source_rx)
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

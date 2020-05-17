use crate::dataset::DS;
use crate::sinks::Sink;
use crate::sources::Source;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

pub struct Environment {
    pub source_runners: Vec<SourceRunner>,
}

struct SourceRunner(Box<dyn Fn() -> ()>);

impl Environment {
    pub fn add_source<S: 'static>(&mut self, source: S) -> DS<S::T>
    where
        S: Source,
    {
        let (source_tx, source_rx) = mpsc::channel::<S::T>();

        let x = SourceRunner(Box::new(move || {
            source.start(source_tx);
        }));

        self.source_runners.push(x);

        DS { input: source_rx }
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

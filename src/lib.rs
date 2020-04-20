use daggy::{Dag, WouldCycle};
use std::iter::once;
use std::sync::mpsc::{Receiver, Sender};

pub trait Step {
    fn name() -> String;
}

pub trait Source<T> {
    fn name() -> String;
    fn start(tx: Sender<T>) -> Result<(), String>;
}

pub trait Sink<T> {
    fn name() -> String;
    fn start(rx: Receiver<T>) -> Result<(), String>;
}

pub struct ConsoleSource {}

impl<T> Source<T> for ConsoleSource {
    fn name() -> String {
        "Console Source".to_owned()
    }

    fn start(rx: Receiver<T>) -> Result<(), String> {
        Ok(())
    }
}

pub struct Processor {}

pub struct Executor<T> {
    name: String,
    source: Box<dyn Source<T>>,
}

impl<T> Executor<T> {
    fn new(name: &str) -> Self {
        let console_source = ConsoleSource {};
        Self {
            name: name.to_string(),
            source: Box::new(console_source),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Executor;

    #[test]
    fn basic_executor_is_created() {
        let executor = Executor::new("Basic Executor");
        assert_eq!(executor.name, "Basic Executor");
    }
}

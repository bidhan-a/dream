use daggy::{Dag, WouldCycle};
use std::iter::once;
use std::sync::mpsc::{Receiver, Sender};

pub trait Step {
    fn name() -> String;
}

pub trait Source {
    type T;
    fn name(self) -> String;
    fn start(self, tx: Sender<Self::T>) -> Result<(), String>;
}

pub trait Sink<T> {
    fn name() -> String;
    fn start(rx: Receiver<T>) -> Result<(), String>;
}

pub struct ConsoleSource {}

impl Source for ConsoleSource {
    type T = Vec<u8>;
    fn name(self) -> String {
        "Console Source".to_owned()
    }

    fn start(self, tx: Sender<Self::T>) -> Result<(), String> {
        Ok(())
    }
}

pub struct Processor {}

pub struct Executor<U: Source> {
    name: String,
    source: U,
}

impl<U: Source> Executor<U> {
    fn new(name: &str, source: U) -> Self {
        Self {
            name: name.to_string(),
            source: source,
        }
    }
}

#[cfg(test)]
mod test {
    use super::ConsoleSource;
    use super::Executor;

    #[test]
    fn basic_executor_is_created() {
        let console_source = ConsoleSource {};
        let executor: Executor<ConsoleSource> = Executor::new("Basic Executor", console_source);
        assert_eq!(executor.name, "Basic Executor");
    }
}

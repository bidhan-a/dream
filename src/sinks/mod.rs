use std::sync::mpsc::Receiver;

pub trait Sink<T> {
    fn name() -> String;
    fn start(rx: Receiver<T>) -> Result<(), String>;
}

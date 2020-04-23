use std::sync::mpsc::Sender;

pub mod stdin;

pub trait Source {
    type T;
    fn name(self) -> String;
    fn start(self, tx: Sender<Self::T>) -> Result<(), String>;
}

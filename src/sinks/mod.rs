use crate::Result;
use std::sync::mpsc::Receiver;

pub mod csv;

pub trait Sink: Clone {
    type T;
    fn name(self) -> String;
    fn start(self, rx: Receiver<Self::T>) -> Result<()>;
}

use crate::{Message, Result};
use std::sync::mpsc::Receiver;

pub mod csv;

pub trait Sink: Clone {
    type T;
    fn name(&self) -> String;
    fn start(self, rx: Receiver<Message<Self::T>>) -> Result<()>
    where
        <Self as Sink>::T: std::clone::Clone;
}

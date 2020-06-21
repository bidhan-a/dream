use crate::{Message, Result};
use std::sync::mpsc::Receiver;

pub mod csv;

/// An interface for defining a data sink.
pub trait Sink: Clone {
    /// The type of the elements expected by the sink.
    type T;

    /// Returns the name of the sink.
    fn name(&self) -> String;

    /// Starts the sink.
    fn start(self, rx: Receiver<Message<Self::T>>) -> Result<()>
    where
        <Self as Sink>::T: std::clone::Clone;
}

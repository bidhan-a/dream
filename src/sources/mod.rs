use crate::{Message, Result};
use std::sync::mpsc::Sender;

pub mod csv;
pub mod stdin;

pub trait Source: Clone {
    type T;
    fn name(self) -> String;
    fn start(self, tx: Sender<Message<Self::T>>) -> Result<()>;
}

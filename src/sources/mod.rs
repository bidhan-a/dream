use crate::{Message, Result};
use std::sync::mpsc::Sender;

pub mod csv;
pub mod stdin;

pub trait Source: Clone {
    type T;
    fn name(self) -> String;
    fn start(self, tx: Sender<Message<Self::T>>) -> Result<()>;
    fn test(self) -> Result<()> {
        Ok(())
    }
}

pub trait SourceWrapper {
    fn test(&mut self);
}

impl<T: Source> SourceWrapper for Option<T> {
    fn test(&mut self) {
        // Option::take() gives owned from non-owned
        self.take().unwrap().test().unwrap();
    }
}

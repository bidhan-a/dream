use daggy::{Dag, WouldCycle};
use std::iter::once;

pub trait Step {
    fn name() -> String;
}

pub trait Source {}

pub trait Sink {}

pub struct Processor {}

pub struct Executor {}

impl Executor {
    fn new() -> Self {
        Self {}
    }
}

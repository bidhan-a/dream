pub mod dataset;
pub mod environment;
pub mod sinks;
pub mod sources;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub enum Message<T> {
    Data(T),
    Terminate,
}

pub trait Processor {
    fn name(&self) -> String;
    fn run(&self);
}

#[cfg(test)]
mod test {}

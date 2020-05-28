pub mod dataset;
pub mod environment;
pub mod flow;
pub mod sinks;
pub mod sources;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub enum Message<T: Clone> {
    Data(T),
    Terminate,
}

#[cfg(test)]
mod test {}

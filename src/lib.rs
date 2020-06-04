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

pub struct Stats {
    records_in: u8,
    records_out: u8,
    bytes_in: u8,
    bytes_out: u8,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            records_in: 0,
            records_out: 0,
            bytes_in: 0,
            bytes_out: 0,
        }
    }
}

#[cfg(test)]
mod test {}

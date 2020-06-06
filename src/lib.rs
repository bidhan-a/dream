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

#[derive(Debug)]
pub struct Stats {
    records_in: u64,
    records_out: u64,
    bytes_in: usize,
    bytes_out: usize,
}

impl Stats {
    pub fn new(records_in: u64, records_out: u64, bytes_in: usize, bytes_out: usize) -> Self {
        Stats {
            records_in,
            records_out,
            bytes_in,
            bytes_out,
        }
    }

    pub fn update(&mut self, stats: Stats) -> bool {
        if stats.records_in == 0 {
            return false;
        }
        self.records_in += stats.records_in;
        self.records_out += stats.records_out;
        self.bytes_in += stats.bytes_in;
        self.bytes_out += stats.bytes_out;
        true
    }

    pub fn get_records_in(&self) -> u64 {
        self.records_in
    }

    pub fn get_records_out(&self) -> u64 {
        self.records_out
    }

    pub fn get_bytes_in(&self) -> usize {
        self.bytes_in
    }

    pub fn get_bytes_out(&self) -> usize {
        self.bytes_out
    }
}

#[cfg(test)]
mod test {}

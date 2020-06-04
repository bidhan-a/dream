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
    bytes_in: usize,
    bytes_out: usize,
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

    pub fn update(&mut self, stats: Stats) -> bool {
        if self.records_in == 0 {
            return false;
        }
        self.records_in += stats.records_in;
        self.records_out += stats.records_out;
        self.bytes_in += stats.bytes_in;
        self.bytes_out += stats.bytes_out;
        true
    }

    pub fn get_records_in(&self) -> u8 {
        self.records_in
    }

    pub fn get_records_out(&self) -> u8 {
        self.records_in
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

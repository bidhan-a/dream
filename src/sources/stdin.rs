use crate::sources::{Sender, Source};
use std::io::{self, BufReader, Read};

const CHUNK_SIZE: usize = 16 * 1024; // 16 kb.

pub struct StdinSource {}

impl Source for StdinSource {
    type T = Vec<u8>;
    fn name(self) -> String {
        "Stdin Source".to_owned()
    }

    fn start(self, tx: Sender<Self::T>) -> Result<(), String> {
        let mut reader: Box<dyn Read> = Box::new(BufReader::new(io::stdin()));
        let mut buffer = [0; CHUNK_SIZE];
        loop {
            let num_read = match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(x) => x,
                Err(_) => break,
            };
            if tx.send(Vec::from(&buffer[..num_read])).is_err() {
                break;
            }
        }
        // no more data, send empty value to signal completion.
        let _ = tx.send(Vec::new());
        Ok(())
    }
}

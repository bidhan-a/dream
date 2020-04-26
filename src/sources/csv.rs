use crate::sources::{Result, Sender, Source};
use csv::{Reader, StringRecord};
use std::fs::File;
use std::io::{self, BufReader, Read};

#[derive(Clone)]
pub struct CSVSource {
    pub filename: String,
}

impl Source for CSVSource {
    type T = StringRecord;
    fn name(self) -> String {
        "Stdin Source".to_owned()
    }

    fn start(self, tx: Sender<Self::T>) -> Result<()> {
        let reader: Box<dyn Read> = if !self.filename.is_empty() {
            Box::new(BufReader::new(File::open(self.filename)?))
        } else {
            Box::new(BufReader::new(io::stdin()))
        };
        let mut rdr = Reader::from_reader(reader);
        for result in rdr.records() {
            match result {
                Err(_) => break,
                Ok(record) => {
                    if tx.send(record).is_err() {
                        break;
                    }
                }
            }
        }

        // no more data, send empty value to signal completion.
        let _ = tx.send(StringRecord::new());
        Ok(())
    }
}

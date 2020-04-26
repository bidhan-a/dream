use crate::sinks::{Receiver, Result, Sink};
use csv::{StringRecord, Writer};
use std::fs::File;
use std::io::{self, Write};

#[derive(Clone)]
pub struct CSVSink {
    filename: String,
}

impl Sink for CSVSink {
    type T = StringRecord;
    fn name(self) -> String {
        "Stdin Source".to_owned()
    }

    fn start(self, rx: Receiver<Self::T>) -> Result<()> {
        let writer: Box<dyn Write> = if !self.filename.is_empty() {
            Box::new(File::create(self.filename)?)
        } else {
            Box::new(io::stdout())
        };
        let mut wtr = Writer::from_writer(writer);

        loop {
            let record: StringRecord = rx.recv().unwrap();
            if record.is_empty() {
                break;
            }
            wtr.write_record(record.iter())?;
        }

        Ok(())
    }
}

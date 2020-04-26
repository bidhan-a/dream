use crate::sinks::{Receiver, Result, Sink};
use csv::{StringRecord, Writer};
use std::fs::File;
use std::io::{self, Write};

#[derive(Clone, Default)]
pub struct CSVSink {
    filename: Option<String>,
}

impl Sink for CSVSink {
    type T = StringRecord;
    fn name(self) -> String {
        "Stdin Source".to_owned()
    }

    fn start(self, rx: Receiver<Self::T>) -> Result<()> {
        let writer: Box<dyn Write> = if let Some(f) = self.filename {
            Box::new(File::create(f)?)
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

        wtr.flush()?;

        Ok(())
    }
}

impl CSVSink {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_owned());
        self
    }
}

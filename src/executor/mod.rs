use crate::sources::Source;
use crate::Result;
// use crossbeam_utils::thread;
use crate::sources::stdin::StdinSource;
use std::sync::{mpsc, Arc};
use std::thread;

#[derive(Debug, Default)]
pub struct Executor<U: Source> {
    pub name: String,
    pub source: U,
}

impl<U: 'static + std::marker::Send + std::marker::Sync + Source> Executor<U> {
    pub fn new(name: &str, source: U) -> Self {
        Self {
            name: name.to_string(),
            source: source,
        }
    }

    pub fn execute(&self) -> Result<()>
    where
        <U as Source>::T: std::marker::Send,
    {
        let (source_tx, source_rx) = mpsc::channel();
        // let (sink_tx, sink_rx) = mpsc::channel();

        // Hard-coded CSV source (for now)
        // let csv_source = CSVSource {
        //     filename: "".to_owned(),
        // };

        // thread::scope(|s| {
        //     s.spawn(|_| {
        //         let _ = self.source.start(source_tx).unwrap();
        //     })
        // })
        // .unwrap();

        let cloned_source = self.source.clone();
        let source_handle = thread::spawn(move || cloned_source.start(source_tx));
        //
        let source_result = source_handle.join().unwrap();
        //
        source_result?;
        // process(self.source);
        Ok(())
    }
}

// fn process<U: std::marker::Sync + Source>(source: U)
// where
//     <U as Source>::T: std::marker::Send,
//     U: std::marker::Send,
// {
//     // thread::scope(|s| {
//     //     // let stdin_source = StdinSource {};
//     //     // let (source_tx, source_rx) = mpsc::channel::<Vec<u8>>();
//     //     s.spawn(|_| {
//     //         let _ = 2 + 2;
//     //         // let _ = stdin_source.start(source_tx).unwrap();
//     //     })
//     // })
//     // .unwrap();
//     let stdin_source = StdinSource {};
//     let (source_tx, source_rx) = mpsc::channel();
//     let read_handle = thread::spawn(move || stdin_source.start(source_tx));
// }

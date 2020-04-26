use crate::sinks::Sink;
use crate::sources::Source;
use crate::Result;
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Default)]
pub struct Executor<U, V>
where
    U: Source,
    V: Sink<T = U::T>,
{
    pub name: String,
    pub source: U,
    pub sink: V,
}

impl<U: 'static, V: 'static> Executor<U, V>
where
    U: std::marker::Send + Source,
    V: std::marker::Send + Sink<T = U::T>,
{
    pub fn new(name: &str, source: U, sink: V) -> Self {
        Self {
            name: name.to_string(),
            source,
            sink,
        }
    }

    pub fn execute(&self) -> Result<()>
    where
        <U as Source>::T: std::marker::Send,
    {
        let (source_tx, source_rx) = mpsc::channel();
        // let (sink_tx, sink_rx) = mpsc::channel();

        let cloned_source = self.source.clone();
        let cloned_sink = self.sink.clone();
        let source_handle = thread::spawn(move || cloned_source.start(source_tx));
        let sink_handle = thread::spawn(move || cloned_sink.start(source_rx));
        let source_result = source_handle.join().unwrap();
        let sink_result = sink_handle.join().unwrap();
        source_result?;
        sink_result?;
        Ok(())
    }
}

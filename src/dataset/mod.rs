use crate::Result;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub struct DataSet<T> {
    input_rx: Option<Receiver<T>>,
    thread: Option<thread::JoinHandle<()>>,
}

impl<T: std::marker::Send + 'static> DataSet<T> {
    pub fn new(input_rx: Receiver<T>) -> DataSet<T> {
        DataSet {
            input_rx: Some(input_rx),
            thread: None,
        }
    }

    pub fn map<U: 'static, F: 'static>(&mut self, f: F) -> DataSet<U>
    where
        F: std::marker::Sync + std::marker::Send + Fn(T) -> U,
        Self: std::marker::Sized,
        U: std::marker::Send,
    {
        let (output_tx, output_rx) = mpsc::channel::<U>();
        let input_rx = self.input_rx.take().unwrap();
        let thread = thread::spawn(move || {
            loop {
                // receive data from input channel.
                let input = input_rx.recv().unwrap();
                let output = f(input);
                if output_tx.send(output).is_err() {
                    break;
                }
            }
        });
        self.thread = Some(thread);
        DataSet::new(output_rx)
    }

    pub fn filter<F: 'static>(&mut self, f: F) -> DataSet<T>
    where
        F: std::marker::Send + Fn(&T) -> bool,
        Self: std::marker::Sized,
    {
        let (output_tx, output_rx) = mpsc::channel::<T>();
        let input_rx = self.input_rx.take().unwrap();
        let thread = thread::spawn(move || {
            loop {
                // receive data from input channel.
                let input = input_rx.recv().unwrap();
                if f(&input) && output_tx.send(input).is_err() {
                    break;
                }
            }
        });
        self.thread = Some(thread);
        DataSet::new(output_rx)
    }
}

impl<T> Drop for DataSet<T> {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

use crate::Result;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub struct DataSet<T> {
    input_rx: Receiver<T>,
}

impl<T: std::marker::Send + 'static> DataSet<T> {
    pub fn new(input_rx: Receiver<T>) -> DataSet<T> {
        DataSet { input_rx }
    }

    pub fn map<U: 'static, F: 'static>(self, f: F) -> DataSet<U>
    where
        F: std::marker::Send + Fn(T) -> U,
        Self: std::marker::Sized,
        U: std::marker::Send,
    {
        let (output_tx, output_rx) = mpsc::channel::<U>();

        // TODO: Need a signal channel that tells when to stop looping.
        thread::spawn(move || {
            loop {
                // receive data from input channel.
                let input = self.input_rx.recv().unwrap();
                let output = f(input);
                if output_tx.send(output).is_err() {
                    break;
                }
            }
        });

        DataSet::new(output_rx)
    }

    pub fn filter<F: 'static>(self, f: F) -> DataSet<T>
    where
        F: std::marker::Send + Fn(&T) -> bool,
        Self: std::marker::Sized,
    {
        let (output_tx, output_rx) = mpsc::channel::<T>();

        // TODO: Need a signal channel that tells when to stop looping.
        thread::spawn(move || {
            loop {
                // receive data from input channel.
                let input = self.input_rx.recv().unwrap();
                if f(&input) && output_tx.send(input).is_err() {
                    break;
                }
            }
        });

        DataSet::new(output_rx)
    }
}

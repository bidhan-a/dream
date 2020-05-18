use crate::Result;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub struct DataSet<T> {
    t: T,
}

impl<T> DataSet<T> {
    // pub fn map<U, F>(self, rx: Receiver<T>, f: F, tx: Sender<U>) -> Result<()>
    pub fn map<U, F>(self, f: F) -> Result<U>
    where
        F: Fn(T) -> U,
        Self: std::marker::Sized,
    {
        // let input: T = rx.recv().unwrap();
        // let result: U = f(input);
        // let _ = tx.send(result);

        let result = f(self.t);
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::DataSet;

    fn plus_1(x: u8) -> u8 {
        x + 1
    }

    #[test]
    fn dataset_works() {
        let ds = DataSet { t: 1 };
        // let f = |x| x + 1;
        let res = ds.map(plus_1).unwrap();
        assert_eq!(res, 2);
    }
}

pub struct DS<T> {
    input_rx: Receiver<T>,
}

impl<T: std::marker::Send + 'static> DS<T> {
    pub fn new(input_rx: Receiver<T>) -> DS<T> {
        DS { input_rx }
    }

    pub fn map<U: 'static, F: 'static>(self, f: F) -> DS<U>
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

        DS::new(output_rx)
    }

    pub fn filter<F: 'static>(self, f: F) -> DS<T>
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

        DS::new(output_rx)
    }
}

use crate::Result;
use std::sync::mpsc::{Receiver, Sender};

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

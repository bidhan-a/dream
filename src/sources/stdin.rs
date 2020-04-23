use crate::sources::{Sender, Source};

pub struct StdinSource {}

impl Source for StdinSource {
    type T = Vec<u8>;
    fn name(self) -> String {
        "Stdin Source".to_owned()
    }

    fn start(self, tx: Sender<Self::T>) -> Result<(), String> {
        Ok(())
    }
}

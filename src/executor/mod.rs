use crate::sources::Source;
use crate::Result;

#[derive(Debug, Default)]
pub struct Executor<U: Source> {
    pub name: String,
    pub source: U,
}

impl<U: Source> Executor<U> {
    pub fn new(name: &str, source: U) -> Self {
        Self {
            name: name.to_string(),
            source,
        }
    }

    pub fn execute() -> Result<()> {
        Ok(())
    }
}

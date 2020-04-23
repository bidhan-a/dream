use crate::sources::Source;

pub struct Executor<U: Source> {
    pub name: String,
    pub source: U,
}

impl<U: Source> Executor<U> {
    pub fn new(name: &str, source: U) -> Self {
        Self {
            name: name.to_string(),
            source: source,
        }
    }
}

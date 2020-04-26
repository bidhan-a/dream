pub mod executor;
pub mod sinks;
pub mod sources;

use daggy::{Dag, WouldCycle};
use executor::Executor;
use sinks::{csv::CSVSink, Sink};
use sources::{csv::CSVSource, Source};
use std::iter::once;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(test)]
mod test {
    use super::CSVSink;
    use super::CSVSource;
    use super::Executor;

    #[test]
    fn basic_executor_is_created() {
        let csv_source = CSVSource::new().with_filename("data/in.csv");
        let csv_sink = CSVSink {
            filename: "data/out.csv".to_owned(),
        };
        let executor: Executor<CSVSource, CSVSink> =
            Executor::new("Basic Executor", csv_source, csv_sink);
        executor.execute().unwrap();
        assert_eq!(executor.name, "Basic Executor");
    }
}

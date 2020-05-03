pub mod dataset;
pub mod executor;
pub mod sinks;
pub mod sources;

use daggy::{Dag, WouldCycle};
use executor::{Executor, E2};
use sinks::{csv::CSVSink, Sink};
use sources::{csv::CSVSource, Source};
use std::iter::once;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(test)]
mod test {
    use super::CSVSink;
    use super::CSVSource;
    use super::Executor;
    use super::E2;

    #[test]
    fn basic_executor_is_created() {
        let csv_source_2 = CSVSource::new().with_filename("data/in.csv");
        let csv_source_3 = CSVSource::new();
        let e: E2 = E2 {
            sources: vec![Box::new(Some(csv_source_2)), Box::new(Some(csv_source_3))],
        };
        for mut s in e.sources {
            s.test();
        }
        // let csv_source = CSVSource::new().with_filename("data/in.csv");
        // let csv_sink = CSVSink::new().with_filename("data/out.csv");
        // let executor: Executor<CSVSource, CSVSink> =
        //     Executor::new("Basic Executor", csv_source, csv_sink);
        // executor.execute().unwrap();
        // assert_eq!(executor.name, "Basic Executor");
    }
}

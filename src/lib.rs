pub mod executor;
pub mod sinks;
pub mod sources;

use daggy::{Dag, WouldCycle};
use executor::Executor;
use sources::{stdin::StdinSource, Source};
use std::iter::once;

#[cfg(test)]
mod test {
    use super::Executor;
    use super::StdinSource;

    #[test]
    fn basic_executor_is_created() {
        let stdin_source = StdinSource {};
        let executor: Executor<StdinSource> = Executor::new("Basic Executor", stdin_source);
        assert_eq!(executor.name, "Basic Executor");
    }
}

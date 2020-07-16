use avro_rs::types::Value;
use chrono::Local;
use dream::dataset::DataSet;
use dream::environment::Environment;
use dream::sinks::print::PrintSink;
use dream::sources::avro::AvroSource;

fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stderr())
        .apply()
        .unwrap();
    let mut env = Environment::new("My Pipeline");

    // Source CSV file: https://raw.githubusercontent.com/Teradata/kylo/master/samples/sample-data/csv/userdata1.csv

    // Add Source.
    let mut dataset: DataSet<Value> = env
        .add_source(AvroSource::new().with_filename("data/twitter.avro"))
        .name("Avro Source");

    let mut to_str: DataSet<String> = dataset
        .map(|row: Value| format!("{:?}", row))
        .name("To Bytes");

    // Print.
    to_str.add_sink(PrintSink::new().to_stderr());

    // Run the pipeline.
    env.run();
}

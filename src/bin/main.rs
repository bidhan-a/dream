use chrono::Local;
use csv::StringRecord;
use dream::dataset::DataSet;
use dream::environment::Environment;
use dream::sinks::csv::CSVSink;
use dream::sources::csv::CSVSource;

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

    // Add Source.
    let mut dataset: DataSet<StringRecord> = env
        .add_source(CSVSource::new().with_filename("data/in.csv"))
        .name("CSV Source");

    // Process data.
    let mut extra_field_1: DataSet<StringRecord> = dataset
        .map(|mut row: StringRecord| {
            row.push_field("extra");
            row
        })
        .name("Extra Field 1");

    // Process data (branches off from extra_field_1).
    let mut extra_field_2: DataSet<StringRecord> = extra_field_1
        .map(|mut row: StringRecord| {
            row.push_field("field1");
            row
        })
        .name("Extra Field 2");

    // Add sink.
    extra_field_2.add_sink_v2(CSVSink::new().with_filename("data/out1.csv"));

    // Process data (branches off from extra_field_2).
    let mut extra_field_3: DataSet<StringRecord> = extra_field_1
        .map(|mut row: StringRecord| {
            row.push_field("field2");
            row
        })
        .name("Extra Field 3");

    // Add sink.
    extra_field_3.add_sink_v2(CSVSink::new().with_filename("data/out2.csv"));

    // Add another sink to extra_field_3.
    extra_field_3.add_sink_v2(CSVSink::new().with_filename("data/out3.csv"));

    /*
     * Diagram
     *
     * CSV Source -> Extra Field 1 -> Extra Field 2 -> Sink
     *                            |-> Extra Field 3 -> Sink
     *                                             |-> Sink
     */

    // Run the pipeline.
    env.run();
}

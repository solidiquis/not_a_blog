use env_logger::Builder;
use log::LevelFilter;

pub fn init_global_logger() {
    let global_log_level = dotenv::var("LOG_LEVEL").unwrap();

    let mut builder = Builder::new();

    let log_level = match global_log_level {
        _ => LevelFilter::Debug
    };

    builder.filter(None, log_level).init()
}

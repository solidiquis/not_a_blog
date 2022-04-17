use env_logger::Builder;
use log::LevelFilter;

pub fn init_global_logger(global_log_level: &str) {
    let mut builder = Builder::new();

    let log_level = match global_log_level {
        _ => LevelFilter::Debug
    };

    builder.filter(None, log_level).init()
}

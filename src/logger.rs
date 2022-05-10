use log::{Record, Level, Metadata};
use colored::*;

pub struct Logger {
    verbose: bool
}

impl Logger {
    pub fn new(is_verbose: bool) -> Self {
        Self {
            verbose: is_verbose
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, meta_data: &Metadata) -> bool {
        if self.verbose {
            meta_data.level() <= Level::Info
        } else {
            meta_data.level() <= Level::Warn
        }
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}| {}", map_error_level_to_colored_string(record.level()), record.args());
        }
    }

    fn flush(&self) {} 
}

fn map_error_level_to_colored_string(level: Level) -> ColoredString {
    match level {
        Level::Info  => "---INFO--- ".to_string().purple(),
        Level::Warn  => "--WARNING--".to_string().yellow(),
        Level::Error => "---ERROR---".to_string().red(),
        _            => "---MISC--- ".to_string().white()
    }
}
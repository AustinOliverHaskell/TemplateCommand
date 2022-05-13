use log::{Record, Level, Metadata, LevelFilter};
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

    pub fn set_global_logger(is_verbose: bool) {
        match log::set_boxed_logger(Box::new(Logger::new(is_verbose))) {
            Ok(()) => {},
            _ => {println!("Failed to initialize logging framework..."); return; } 
        }
        log::set_max_level(if is_verbose { LevelFilter::Info } else { LevelFilter::Warn });
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
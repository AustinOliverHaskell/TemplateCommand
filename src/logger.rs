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
            match record.level() {
                Level::Error => println!("--- {} --- {}", record.level().as_str().red(), record.args()), 
                Level::Warn => println!("--- {} --- {}", record.level().as_str().yellow(), record.args()), 
                Level::Info => println!("--- {} --- {}", record.level().as_str().purple(), record.args()), 
                _ => println!("--- {} --- {}", record.level(), record.args())
            }
        }
    }

    fn flush(&self) {} 
}
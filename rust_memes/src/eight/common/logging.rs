use log::{Metadata, Record};

pub struct EightLogger;

impl log::Log for EightLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true // Always log everything
    }

    fn log(&self, record: &Record) {
        let args = record.args();

        match record.metadata().level() {
            l => {
                println!("[{}] {}", l, args);
            } // Level::Info => {}
              // Level::Error => {}
              // Level::Warn => {}
              // Level::Debug => {}
              // Level::Trace => {}
        }
    }

    fn flush(&self) {}
}

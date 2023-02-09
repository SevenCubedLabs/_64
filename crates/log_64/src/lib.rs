#![no_std]
pub use log::*;

pub fn init() {
    static LOGGER: StdLogger = StdLogger;
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("failed to init logs");
    log::info!("Greetz");
}

struct StdLogger;

impl log::Log for StdLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            #[cfg(all(feaature = "std", feature = "log"))]
            println!("[{:>5}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

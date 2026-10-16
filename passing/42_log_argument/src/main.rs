use std::{
    cell::Cell,
    sync::atomic::{AtomicU32, Ordering},
};

struct Logger;
static RECORDS: AtomicU32 = AtomicU32::new(0);

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        assert_eq!(record.args().to_string(), "selected value: 7");
        RECORDS.fetch_add(1, Ordering::Relaxed);
    }

    fn flush(&self) {}
}

fn record(flag: bool, evaluated: &Cell<u32>) {
    log::info!("selected value: {}", {
        evaluated.set(evaluated.get() + 1);
        if flag { 7 } else { 9 }
    });
}

fn main() {
    log::set_logger(&Logger).expect("install the sample logger");
    log::set_max_level(log::LevelFilter::Info);
    let evaluated = Cell::new(0);
    record(std::hint::black_box(true), &evaluated);
    assert_eq!(evaluated.get(), 1);
    assert_eq!(RECORDS.load(Ordering::Relaxed), 1);
}

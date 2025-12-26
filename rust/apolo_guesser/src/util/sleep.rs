use std::{thread, time::Duration};

pub fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

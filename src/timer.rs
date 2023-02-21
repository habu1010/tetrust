use std::sync::{Condvar, Mutex};
use std::time::Duration;

pub struct Timer {
    reset_timer: Mutex<bool>,
    condvar: Condvar,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            reset_timer: Mutex::new(false),
            condvar: Condvar::new(),
        }
    }

    pub fn wait(&self, duration: Duration) {
        loop {
            let reset_timer = self.reset_timer.lock().unwrap();
            let (mut reset_timer, timeout_result) = self
                .condvar
                .wait_timeout_while(reset_timer, duration, |reset_timer| !*reset_timer)
                .unwrap();
            *reset_timer = false;
            if timeout_result.timed_out() {
                return;
            }
        }
    }

    pub fn reset(&self) {
        let mut reset_timer = self.reset_timer.lock().unwrap();
        *reset_timer = true;
        self.condvar.notify_all();
    }
}

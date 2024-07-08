
use crate::parallel::exponential::ExponentialBackoff;
use std::sync::atomic::{AtomicBool, Ordering};

struct SpinMutex {
    _flag: AtomicBool
}

impl SpinMutex {
    pub fn new(is_locked: bool) -> Self {
        SpinMutex {
            _flag: AtomicBool::new(is_locked)
        }
    }

    pub fn lock(&self) {
        let mut expected = false;
        if self._flag.compare_exchange_weak(expected, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            let mut backoff: ExponentialBackoff = ExponentialBackoff::new();
            loop {
                expected = false;
                if self._flag.compare_exchange_weak(expected, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    break;
                }
                backoff.wait();
            }
        }
    }

    pub fn try_lock(&self) -> bool {
        let expected = false;
        return self._flag.compare_exchange_weak(expected, true, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }

    pub fn unlock(&self) {
        self._flag.store(false, Ordering::Release);
    }
}
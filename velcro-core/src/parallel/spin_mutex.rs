
use std::{mem, sync::atomic::AtomicBool};

struct SpinMutex {
    _flag: AtomicBool
}

impl SpinMutex {
    pub fn new(isLocked: &bool) -> Self {
        SpinMutex {
            _flag: 
        }
    }
}


use std::thread;

const MAX_PAUSE_LOOPS: i32  = 32;


pub struct ExponentialBackoff {
    _count: i32
}

impl ExponentialBackoff {
    pub fn new() -> Self {
        ExponentialBackoff {
            _count: 1
        }
    }

    pub fn wait(&mut self) {
        if self._count <= MAX_PAUSE_LOOPS {
            for _ in 0..self._count  {
               thread::yield_now();
            }
            self._count <<= 1;
        } else {
            thread::yield_now();
        }
    }

    pub fn reset(&mut self) {
        self._count = 1;
    }
}
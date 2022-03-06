use std::sync::Condvar;
use std::sync::Mutex;

pub mod round;
pub mod slice_shuffler;

pub struct Monitor<T> {
    pub m: Mutex<T>,
    pub cv: Condvar,
}

impl<T> Monitor<T> {
    pub fn new(val: T) -> Self {
        Self {
            m: Mutex::new(val),
            cv: Condvar::new(),
        }
    }
}

use std::fmt;
use std::thread;
use std::time::Duration;

pub struct Round<const MAX: u32 = 10> {}

impl<const MAX: u32> Round<MAX> {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start<R>(&mut self, mut runnable: impl FnMut() -> R, sleep: Option<Duration>)
    where
        R: fmt::Debug,
    {
        for no in 1..=MAX {
            println!("/////////// ROUND {no} ///////////\n");

            let ret = runnable();

            println!("\n/////////// RES {:?} ///////////\n", ret);

            if let Some(d) = sleep {
                thread::sleep(d);
            }
        }
    }
}

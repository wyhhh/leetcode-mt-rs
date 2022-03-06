use core::fmt;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::slice::Iter;
use std::thread;
pub struct SliceShuffler<'a, T> {
    slice: &'a mut [T],
}

impl<'a, T> SliceShuffler<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        Self { slice }
    }

    pub fn with_shuffle(slice: &'a mut [T]) -> Self {
        let mut slf = Self::new(slice);
        slf.shuffle();
        slf
    }

    pub fn shuffle(&mut self) {
        self.slice.shuffle(&mut thread_rng());
    }

    pub fn iter(&self) -> Iter<T> {
        self.slice.iter()
    }

    pub fn run<F, R>(&mut self, run: F)
    where
        F: FnMut(T) -> R + Send + 'static + Clone,
        R: fmt::Debug + Send + 'static,
        T: fmt::Debug + Send + 'static + Clone,
    {
        let mut handles = Vec::with_capacity(self.slice.len());
        self.shuffle();
        println!("shuffle res: {:?}", self);

        for x in &*self {
            let x = x.clone();
            let mut f = run.clone();

            handles.push(thread::spawn(move || f(x)));
        }

        for h in handles {
            let res = h.join().unwrap();
            println!("thread res: [[ {:?} ]]", res);
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for SliceShuffler<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.slice)
    }
}

impl<'a, T> IntoIterator for &'a SliceShuffler<'_, T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

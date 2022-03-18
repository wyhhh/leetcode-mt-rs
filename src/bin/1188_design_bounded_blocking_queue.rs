use leetcode_mt_rs::parking_lot;
use rand::thread_rng;
use rand::Rng;
use std::collections::VecDeque;

// https://leetcode-cn.com/problems/design-bounded-blocking-queue/
fn main() {
    run(&MonitorQueue::new(20));
}

fn run(q: &(impl BoundedBlockingQueue + Sync)) {
    let mut assert_size = 0;

    crossbeam_utils::thread::scope(|s| {
        for (i, enqueue, n) in (1..=201).map(|x| {
            let r = thread_rng().gen_range(-100..100);
            (x, x % 2 == 1, r)
        }) {
            if enqueue {
                assert_size += 1;
            } else {
                assert_size -= 1;
            }

            println!(
                "{i}. [[ {} ]] => [ {} ]",
                if enqueue { "ENQUEUE" } else { "DEQUEUE" },
                if enqueue {
                    n.to_string()
                } else {
                    "".to_string()
                },
            );

            s.spawn(move |_| {
                if enqueue {
                    q.enqueue(n);
                } else {
                    let ele = q.dequeue();
                    println!("dequeue element: {ele}");
                }
            });
        }
    })
    .unwrap();

    println!("////////// END SIZE: {:?} /////////", q.size());
    assert_eq!(assert_size, q.size());
}

trait BoundedBlockingQueue {
    fn new(cap: usize) -> Self;
    fn enqueue(&self, ele: i32);
    fn dequeue(&self) -> i32;
    fn size(&self) -> usize;
}

struct MonitorQueue {
    monitor: parking_lot::Monitor<Vec<i32>>,
    init_cap: usize,
}

impl BoundedBlockingQueue for MonitorQueue {
    fn new(cap: usize) -> Self {
        Self {
            monitor: parking_lot::Monitor::new(Vec::with_capacity(cap)),
            init_cap: cap,
        }
    }

    fn enqueue(&self, ele: i32) {
        {
            let mut q = self.monitor.m.lock();

            // here we must use while avoiding other enqueue
            // got lock BUT the queue is full
            while q.len() == self.init_cap {
                self.monitor.c.wait(&mut q);
            }

            assert!(q.len() < self.init_cap);

            q.push(ele);

            // dropped lock
        }

        // because the q has just two state:
        // => FULL, n enqueue threads are blocking maybe;
        // => NON-FULL, m dequeue threads are blocking maybe
        // but, if when at FULL state, the one of dequeue thread
        // wakeup ALL the other threads, so the all BLOCKING
        // enqueue threads are awaken. thus, when we call
        // enqueue, there are likely some threads DEQUEUE blocking,
        // we just notify one of it will be OK.
        self.monitor.c.notify_one();
    }

    fn dequeue(&self) -> i32 {
        let ret = {
            let mut q = self.monitor.m.lock();

            // N.B. here must use whlie loop
            // because when a dequeue thread awaken
            // all the other threads including this
            // had been awaken, but it is empty, so
            // next line would panic.
            while q.is_empty() {
                self.monitor.c.wait(&mut q);
            }

            q.pop().unwrap()
            // unlock here
        };

        // here we need notify all
        self.monitor.c.notify_all();

        ret
    }

    fn size(&self) -> usize {
        self.monitor.m.lock().len()
    }
}

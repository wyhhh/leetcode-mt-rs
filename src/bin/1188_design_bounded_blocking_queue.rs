use leetcode_mt_rs::parking_lot;
use rand::thread_rng;
use rand::Rng;
use std::thread;
use std::time::Duration;

// https://leetcode-cn.com/problems/design-bounded-blocking-queue/
fn main() {
    run(&MonitorQueue::new(20));

	// let q = &MonitorQueue::new(1);

	// crossbeam_utils::thread::scope(|s| {
	// 	s.spawn(move |_| {
	// 		thread::sleep(Duration::from_millis(200));
	// 		q.enqueue(1);
	// 	});
	// 	s.spawn(move |_| {
	// 		q.dequeue();
	// 	});
	// 	s.spawn(move |_| {
	// 		thread::sleep(Duration::from_millis(200));
	// 		q.enqueue(1);
	// 	});
	// 	s.spawn(move |_| {
	// 		thread::sleep(Duration::from_millis(600));
	// 		q.enqueue(1);
	// 	});
	// 	s.spawn(move |_| {
	// 		thread::sleep(Duration::from_millis(800));
	// 		q.enqueue(1);
	// 	});
	// 	}).unwrap();
}

fn run(q: &(impl BoundedBlockingQueue + Sync)) {
    let mut assert_size = 0;

    crossbeam_utils::thread::scope(|s| {
        for (i, enqueue, n) in (1..=11).map(|x| {
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
					thread::sleep(Duration::from_millis(50));
                    println!("-> enqueue element: {n}");
                    q.enqueue(n);
                } else {
                    let ele = q.dequeue();
                    println!("<- dequeue element: {ele}");
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
			println!("enqueue coming!");

            // here we must use while avoiding other enqueue
            // got lock BUT the queue is full
            while q.len() == self.init_cap {
				println!("enqueue waiting..");
                self.monitor.c.wait(&mut q);
				println!("---enqueue wakeup---");
            }

            assert!(q.len() < self.init_cap);

            q.push(ele);
			println!("push ok");

            // dropped lock
        }

		// thread::sleep(Duration::from_secs(1));
        self.monitor.c.notify_all();
		println!("enqueue wakeup one");
    }

    fn dequeue(&self) -> i32 {
        let ret = {
            let mut q = self.monitor.m.lock();
			println!("dequeue coming!");

            // N.B. here must use whlie loop
            // because when a dequeue thread awaken
            // all the other threads including this
            // had been awaken, but it is empty, so
            // next line would panic.
            while q.is_empty() {
				println!("dequeue waiting..");
                self.monitor.c.wait(&mut q);
				println!("*** dequeue wake up ***");
            }

			println!("pop ok");
            q.pop().unwrap()
            // unlock here
        };

        self.monitor.c.notify_all();

		println!("dequeue wakeup one");
        ret
    }

    fn size(&self) -> usize {
        self.monitor.m.lock().len()
    }
}

use leetcode_mt_rs::round::Round;
use leetcode_mt_rs::slice_shuffler::SliceShuffler;
use leetcode_mt_rs::Monitor;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std_semaphore::Semaphore;
use parking_lot::Mutex;

// https://leetcode-cn.com/problems/print-in-order/
fn main() {
    // solution_with_monitor();
    // solution_with_semaphore();
    // solution_with_sleep();
	solution_with_mutex();
}

fn solution_with_monitor() {
    let mut data = [1, 2, 3];
    let mut ss = SliceShuffler::new(&mut data);
    // we test 10 times
    let mut r: Round<10> = Round::new();

    r.start(
        || {
            let monitor = Arc::new(Monitor::new(1));
            let ans = Arc::new(AtomicU32::new(1));

            ss.run(move |no| {
                let mut g = monitor.m.lock().unwrap();

                // if we don't get the right answer, just waiting...
                while *g != no {
                    g = monitor.cv.wait(g).unwrap();
                }

                println!("print no {no}!");
                let ans = ans.fetch_add(1, Ordering::SeqCst);
                assert_eq!(no, ans);

                *g += 1;
                // every one who got the right anwser just notify others
                monitor.cv.notify_all();

                format!("thread {no} res: {:?}", *g - 1)
            })
        },
        Some(Duration::from_millis(2000)),
    );
}

fn solution_with_semaphore() {
    let mut data = [1, 2, 3];
    let mut ss = SliceShuffler::new(&mut data);
    // we test 10 times
    let mut r: Round<10> = Round::new();

    r.start(
        || {
            let s = Arc::new(Semaphore::new(0));
            let s2 = Arc::new(Semaphore::new(0));
            let ans = Arc::new(AtomicU32::new(1));

            ss.run(move |no| {
                match no {
                    1 => {
                        // when the no 1 thread coming, we directly print it.
                        println!("print no {no}!");
                        let ans = ans.fetch_add(1, Ordering::SeqCst);
                        assert_eq!(no, ans);

                        // and then release the semaphore one
                        s.release();
                    }
                    2 => {
                        // at no 2, we just acquire blockly for the semaphore one ready(just release adding one).
                        s.acquire();

                        println!("print no {no}!");
                        let ans = ans.fetch_add(1, Ordering::SeqCst);
                        assert_eq!(no, ans);

                        // last we release the s2 for no 3 printing
                        s2.release();
                    }
                    3 => {
                        // same as the no 2
                        s2.acquire();

                        println!("print no {no}!");
                        let ans = ans.fetch_add(1, Ordering::SeqCst);
                        assert_eq!(no, ans);
                    }
                    _ => unreachable!(),
                }
            })
        },
        Some(Duration::from_millis(2000)),
    );
}

fn solution_with_mutex() {
	let m = &Mutex::new(1);

	crossbeam_utils::thread::scope(|s| {
		for n in 1..=3 {
			s.spawn(move |_| {
				let mut no = m.lock();

				if *no == n {
					println!("{n} print!");
					*no += 1;
				}
			});
		}
	}).unwrap();
}

// just use thread::sleep
fn solution_with_sleep() {
    let mut data = [1, 2, 3];
    let mut ss = SliceShuffler::new(&mut data);
    // we test 10 times
    let mut r: Round<10> = Round::new();

    r.start(
        || {
            let ans = Arc::new(AtomicU32::new(1));

            ss.run(move |no| match no {
                1 => {
                    println!("print no {no}!");
                    let ans = ans.fetch_add(1, Ordering::SeqCst);
                    assert_eq!(no, ans);
                }
                2 => {
                    thread::sleep(Duration::from_millis(50));

                    println!("print no {no}!");
                    let ans = ans.fetch_add(1, Ordering::SeqCst);
                    assert_eq!(no, ans);
                }
                3 => {
                    thread::sleep(Duration::from_millis(100));

                    println!("print no {no}!");
                    let ans = ans.fetch_add(1, Ordering::SeqCst);
                    assert_eq!(no, ans);
                }
                _ => unreachable!(),
            })
        },
        Some(Duration::from_millis(2000)),
    );
}

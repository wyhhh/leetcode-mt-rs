use leetcode_mt_rs::round::Round;
use leetcode_mt_rs::slice_shuffler::SliceShuffler;
use leetcode_mt_rs::Monitor;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

fn main() {
	solution_with_monitor();
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
                assert_eq!(no, ans.load(Ordering::SeqCst));
                ans.fetch_add(1, Ordering::SeqCst);

                *g += 1;
				// every one who got the right anwser just notify others
                monitor.cv.notify_all();

                format!("thread {no} res: {:?}", *g - 1)
            })
        },
        Some(Duration::from_millis(2000)),
    );
}
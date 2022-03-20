use conc::Sema;
use parking_lot::Mutex;
use std::sync::Barrier;
use std::time::Duration;

// https://leetcode-cn.com/problems/the-dining-philosophers/
fn main() {
    // deadlock_error_demonstration_with_mutex();
    // solution_with_mutexes_and_sema();
    solution_with_serialization();
    // soluation_with_first_do_once();
}

/// when each one all gets the left or right fork, it will be deadlock 😒
#[deprecated]
fn deadlock_error_demonstration_with_mutex() {
    let forks: &[Mutex<()>; 5] = &Default::default();
    let b = &Barrier::new(5);

    crossbeam_utils::thread::scope(|s| {
        for i in 0..5 {
            s.spawn(move |_| {
                // get right hand fork index of philosopher
                let right = || if i == 0 { 4 } else { i - 1 };

                for _ in 0..1 {
                    let left = forks[i].lock();
                    println!("--- {i} get LEFT fork! ---");

                    let right = forks[right()].lock();
                    println!("--- {i} get RIGHT fork! ---");

                    println!("{i} eating!");

                    drop(left);
                    println!("=== {i} put LEFT fork! ===");

                    drop(right);
                    println!("=== {i} put RIGHT fork! ===\n");

                    // synchronize each philosopher
                    b.wait();
                }
            });
        }
    })
    .unwrap();
}

// so how we sovle the deadlock?
// we can think the MAX locks which can be acquired
// are FOUR, so we can set a semephore which the max
// permits is 4, but when some thread gets the right
// hand fork mutex, we release one, that the "link"
// was resolved, and the problem was gone!
fn solution_with_mutexes_and_sema() {
    let forks: &[Mutex<()>; 5] = &Default::default();
    let sema = &Sema::new(4);
    let b = &Barrier::new(5);
    let philosophers = ["🧔", "👱", "🧓", "👦", "👨"];

    crossbeam_utils::thread::scope(|s| {
        for i in 0..5 {
            let philosopher = philosophers[i];

            s.spawn(move |_| {
                // get right hand fork index of philosopher
                let right = || if i == 0 { 4 } else { i - 1 };

                for _ in 0..100 {
                    // the max permits is four, so when
                    // the fifth one wants to coming, it
                    // should be blocked.
                    sema.acquire();

                    let left = forks[i].lock();
                    println!("--- {i}({philosopher}) get LEFT fork! ---");

                    let right = forks[right()].lock();

                    println!("--- {i}({philosopher}) get RIGHT fork! ---");

                    println!("{i}({philosopher}) eating! 🍚🍚🍚");

                    drop(left);
                    println!("=== {i}({philosopher}) put LEFT fork! ===");

                    drop(right);
                    println!("=== {i}({philosopher}) put RIGHT fork! ===\n");

                    // but, when some thread get the last remaining right lock
                    // it acquires, and thus go ahead, the two locks in it
                    // will be released, and the procedure get going on.
                    // what a wonderful design! LOL :)

                    // whatever, when that is not going to one-shot get
                    // five permits, the permits will be always greater
                    // than one, there is always at least one door opening
                    // for threads going on.
                    sema.release();

                    // synchronize each philosopher
                    let last = b.wait();

                    // we need reset the permits to ensure
                    // there are always four permits at MAX
                    // won't would be less it because of the
                    // unordered scheduling
                    // but, we just reset once
                    if last.is_leader() {
                        sema.reset();
                    }

                    // N.B. because of the above reset, we
                    // need ensure all five threads synchronizing
                    // to go next round, if not, the permits will
                    // be disordered and cause the deadlock
                    b.wait();
                }
            });
        }
    })
    .unwrap();
}

/// BEST performance solution
fn soluation_with_first_do_once() {
    let forks: &[Mutex<()>; 5] = &Default::default();
    let b = &Barrier::new(5);

    crossbeam_utils::thread::scope(|s| {
        for i in 0..5 {
            s.spawn(move |_| {
                // get right hand fork index of philosopher
                let right = || if i == 0 { 4 } else { i - 1 };
                let do_remaining = |l, r| {
                    println!("{i} eating!");
                    drop(l);
                    println!("=== {i} put LEFT fork! ===");
                    drop(r);
                    println!("=== {i} put RIGHT fork! ===\n");
                };

				
                for _ in 0..2 {
                    if i == 0 {
                        // std::thread::sleep(Duration::from_secs(1));

                        // Here we inverse the order left to right
                        // in worst situation, the other forks
                        // 1, 2, 3, 4 already be taken, thus lock 0
                        // we be blocked because of the lock 4 taken
                        // so next must be lock 0 be taken by no.1
                        // philosopher,and thus the ring is break
                        // down one by one.
                        // Or if the lock 4 first be taken by philosapher
                        // 0, so the p.4 can not get it, blocking
                        // and whatever lock 0 taken by p.1 or p.0
                        // the ring will be released too.
                        // So there is no possibility CAUSING deadlock
                        let r = forks[4].lock();
                        println!("--- {i} get RIGHT fork! ---");
                        let l = forks[0].lock();
                        println!("--- {i} get LEFT fork! ---");

                        do_remaining(l, r);
                    } else {
                        let l = forks[i].lock();
                        println!("--- {i} get LEFT fork! ---");

                        let r = forks[right()].lock();
                        println!("--- {i} get RIGHT fork! ---");

                        do_remaining(l, r);
                    }

                    // synchronize each philosopher
                    b.wait();
                }
            });
        }
    })
    .unwrap();
}

/// ac, but slow
fn solution_with_serialization() {
    let lock = &Mutex::new(());
    let b = &Barrier::new(5);

    crossbeam_utils::thread::scope(|s| {
        for i in 0..5 {
            s.spawn(move |_| {
                for _ in 0..1 {
                    {
                        let _g = lock.lock();
                        println!("--- {i} get LEFT fork! ---");
                        println!("--- {i} get RIGHT fork! ---");
                        println!("{i} eating!");
                        println!("=== {i} put LEFT fork! ===");
                        println!("=== {i} put RIGHT fork! ===\n");
                    }
                    // synchronize each philosopher
                    b.wait();
                }
            });
        }
    })
    .unwrap();
}

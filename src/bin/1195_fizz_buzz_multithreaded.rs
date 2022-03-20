use std::sync::Barrier;

// https://leetcode-cn.com/problems/fizz-buzz-multithreaded/
fn main() {
    solution_with_barrier();
}

fn solution_with_barrier() {
    let b = &Barrier::new(4);

    crossbeam_utils::thread::scope(|s| {
        for no in 1..=4 {
            s.spawn(move |_| {
                for n in 1..=30 {
                    match no {
                        1 => {
                            if n % 3 == 0 && n % 5 != 0 {
                                println!("{n}: fizz");
                            }
                        }
                        2 => {
                            if n % 5 == 0 && n % 3 != 0 {
                                println!("{n}: buzz");
                            }
                        }
                        3 => {
                            if n % 15 == 0 {
                                println!("{n}: fizzbuzz");
                            }
                        }
                        4 => {
                            if n % 3 != 0 && n % 5 != 0 {
                                println!("{n}: {n}");
                            }
                        }
                        _ => unreachable!(),
                    }
                    // we just wait here
                    b.wait();
                }
            });
        }
    })
    .unwrap();
}

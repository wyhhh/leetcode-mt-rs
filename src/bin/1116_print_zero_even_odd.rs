use leetcode_mt_rs::Monitor;

// https://leetcode-cn.com/problems/print-zero-even-odd/
fn main() {
    solution_with_monitor();
}

fn solution_with_monitor() {
    const N: u32 = 20;
    // we need a "turn" variable for tracking the right state of execution..
    enum Turn {
        Zero { last_is_odd: bool },
        Odd,
        Even,
    }
    let m = Monitor::new(Turn::Zero { last_is_odd: false });

    crossbeam_utils::thread::scope(|s| {
        s.spawn(|_| {
            for _ in 0..N {
                let mut g = m.m.lock().unwrap();

                // whenver it isn't turn::zero, we wait
                while !matches!(*g, Turn::Zero { .. }) {
                    g = m.cv.wait(g).unwrap();
                }

                println!("0");

                // when we got here, update the state by `last_is_odd`
                match *g {
                    Turn::Zero { last_is_odd } => {
                        if last_is_odd {
                            *g = Turn::Even;
                        } else {
                            *g = Turn::Odd
                        }
                    }
                    _ => unreachable!(),
                }

                m.cv.notify_all();
            }
        });
        s.spawn(|_| {
            for n in (2..=N).step_by(2) {
                let mut g = m.m.lock().unwrap();

                while !matches!(*g, Turn::Even) {
                    g = m.cv.wait(g).unwrap();
                }

                println!("{n}");

                // update next state which must be zero
                *g = Turn::Zero { last_is_odd: false };

                m.cv.notify_all();
            }
        });
        s.spawn(|_| {
            for n in (1..=N).step_by(2) {
                let mut g = m.m.lock().unwrap();

                while !matches!(*g, Turn::Odd) {
                    g = m.cv.wait(g).unwrap();
                }

                println!("{n}");

                // update next state which must be zero
                *g = Turn::Zero { last_is_odd: true };

                m.cv.notify_all();
            }
        });
    })
    .unwrap();
}

use std::sync::Barrier;

fn main() {
    solution_with_barrier();
}

fn solution_with_barrier() {
    const N: u32 = 10;
    let b = Barrier::new(2);
    let b2 = Barrier::new(2);

    crossbeam_utils::thread::scope(|s| {
        s.spawn(|_| {
            for _ in 0..N {
                // we use two barriers
                println!("foo");
                b.wait();
                b2.wait();
            }
        });
        s.spawn(|_| {
            for _ in 0..N {
                b.wait();
                // here print "bar" must AFTER the fisrt print "foo"
                println!("bar\n");
                // second for synchorinizing
                b2.wait();
            }
        });
    })
    .unwrap();
}

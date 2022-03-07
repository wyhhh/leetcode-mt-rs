use conc::Sema;
use leetcode_mt_rs::slice_shuffler::SliceShuffler;

// https://leetcode-cn.com/problems/building-h2o/
fn main() {
    solution_with_sema();
}

fn solution_with_sema() {
    let h20 = make_h20(4);
    // we just need two Semaphores, one for H, another for O
    let h = &Sema::new(2);
    let o = &Sema::new(2);
    println!("answer:");

    crossbeam_utils::thread::scope(|s| {
        for k in h20 {
            s.spawn(move |_| {
                match k {
                    b'H' => {
                        // each time we acquire one H molecule
                        h.acquiren(1);
                        print!("H");
                        // and release one O molecule
                        o.releasen(1);
                    }
                    b'O' => {
                        // On O side, we need two O molecules unless waiting until
                        // satifying by H side.
                        o.acquiren(2);
                        print!("O");
                        // end, we release two Hs.
                        h.releasen(2);
                    }
                    _ => unreachable!(),
                }
            });
        }
    })
    .unwrap();
}

fn make_h20(n: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(n + 2 * n);

    for _ in 0..n {
        v.push(b'H');
        v.push(b'H');
        v.push(b'O');
    }

    SliceShuffler::with_shuffle(&mut v);

    println!("shuffled: ");
    for x in &v {
        print!("{}", *x as char);
    }
    println!("\n");
    v
}

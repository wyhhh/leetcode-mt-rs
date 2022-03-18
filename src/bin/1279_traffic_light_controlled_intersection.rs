use parking_lot::Mutex;
use rand::thread_rng;
use rand::Rng;
use leetcode_mt_rs::slice_shuffler::SliceShuffler;

fn main() {
    solution_with_mutex();
}

fn solution_with_mutex() {
    // 1 & 2 dir are green
    let m = &Mutex::new(true);

    crossbeam_utils::thread::scope(|s| {
        for (id, dir) in Cars::new(4) {
            s.spawn(move |_| {
                let mut is_a = m.lock();

                if !(*is_a && matches!(dir, 1 | 2)) {
                    *is_a = false;
                    println!("---------> Turn to 3 & 4");
                }

                println!("[{id}] car has passed dir {dir}");
            });
        }
    })
    .unwrap();
}

struct Cars {
    amt: u32,
	ids: Vec<u32>,
}

impl Cars {
    fn new(amt: u32) -> Self {
		let mut ids: Vec<u32> = (1..=amt).collect();
		SliceShuffler::with_shuffle(&mut ids);
			
        Self { amt, ids }
    }
}

impl Iterator for Cars {
    // id, direction
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.amt == 0 {
            None
        } else {
            self.amt -= 1;
			let id = unsafe {self.ids.get_unchecked(self.amt as usize)};

            Some((
				*id,
                thread_rng().gen_range(1..=4),
            ))
        }
    }
}

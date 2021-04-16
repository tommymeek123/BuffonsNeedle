use std::io::{stdout,stdin,Write};
use threadpool::ThreadPool;
use rand::prelude::*;
use std::sync::mpsc::channel;

#[derive(Copy, Clone)]
pub struct Experiment {
    pub needle_len: f64,
    pub line_dist: f64,
    pub num_needles: u64,
    pub num_threads: usize,
    needles_per_thread: u64,
}

impl Experiment {
    pub const MAX_THREADS: usize = 200;

    pub fn new() -> Self {
        Experiment {
            needle_len: 0.0,
            line_dist: 0.0,
            num_needles: 0,
            num_threads: 0,
            needles_per_thread: 0,
        }
    }

    pub fn input(&mut self) {
        // Get needle length.
        print!("Please enter the length of the needles: ");
        stdout().flush().expect("Failed to flush stdout.");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read user input.");
        self.needle_len = buf.trim().parse::<f64>().expect("Failed to parse input.");

        // Get line distance.
        print!("Please enter the distance between the lines: ");
        stdout().flush().expect("Failed to flush stdout.");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read user input.");
        self.line_dist = buf.trim().parse::<f64>().expect("Failed to parse input.");

        // Get the number of needles.
        print!("Please enter the total number of needles: ");
        stdout().flush().expect("Failed to flush stdout.");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read user input.");
        self.num_needles = buf.trim().parse::<u64>().expect("Failed to parse input.");

        // Get the number of threads.
        print!("Please enter the number of threads: ");
        stdout().flush().expect("Failed to flush stdout.");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read user input.");
        self.num_threads = buf.trim().parse::<usize>().expect("Failed to parse input.");

        // Calculate the number of needles to be dropped by each thread.
        self.needles_per_thread = self.num_needles / self.num_threads as u64;
    }

    pub fn go(self) -> f64 {
        let mut rng = rand::thread_rng();
        let pool = ThreadPool::new(Experiment::MAX_THREADS);
        let (sender, receiver) = channel();

        for _ in 0..self.num_threads {
            let cloned_sender = sender.clone();
            let mut small_rng = SmallRng::from_rng(&mut rng).unwrap();
            pool.execute(move || {
                cloned_sender.send(self.sim(&mut small_rng)).unwrap();
            });
        }

        drop(sender);
        let mut hits: u64 = 0;
        for result in receiver {
            hits += result;
        }
        self.calculate(hits)
    }

    fn calculate(self, hits: u64) -> f64 {
        2.0 * self.needle_len * self.num_needles as f64 / (self.line_dist * hits as f64)
    }

    fn sim(self,  rng: &mut SmallRng) -> u64 {
        let mut hits: u64 = 0;
        for _ in 0..self.needles_per_thread {
            let y = rng.gen_range(0.0..self.line_dist);
            let theta: f64 = rng.gen_range(0.0..180.0);
            if self.needle_len * theta.to_radians().sin() + y > self.line_dist {
                hits += 1;
            }
        }
        hits
    }
}
use std::io::*;
use threadpool::ThreadPool;
// use std::thread;
//use rand::rngs::SmallRng;
//use rand::{Rng,SeedableRng};
use rand::prelude::*;
use std::sync::mpsc::channel;

#[derive(Copy, Clone)]
pub struct Experiment {
    pub needle_len: f64,
    pub line_dist: f64,
    pub num_needles: u64,
    pub num_threads: usize,
}

impl Experiment {
    pub const MAX_THREADS: usize = 250;

    pub fn new() -> Self {
        Experiment {
            needle_len: 0.0,
            line_dist: 0.0,
            num_needles: 0,
            num_threads: 0,
        }
    }

    pub fn input(&mut self) {
        // get needle length
        print!("Please enter the length of the needles: ");
        stdout().flush().expect("Failed to flush stdout.");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read user input.");
        self.needle_len = buf.trim().parse::<f64>().expect("Failed to parse input.");

        // get line distance
        print!("Please enter the distance between the lines: ");
        stdout().flush().expect("Failed to flush stdout.");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read user input.");
        self.line_dist = buf.trim().parse::<f64>().expect("Failed to parse input.");

        // get the number of needles
        print!("Please enter the total number of needles: ");
        stdout().flush().expect("Failed to flush stdout.");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read user input.");
        self.num_needles = buf.trim().parse::<u64>().expect("Failed to parse input.");

        // get the number of threads
        print!("Please enter the number of threads: ");
        stdout().flush().expect("Failed to flush stdout.");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read user input.");
        self.num_threads = buf.trim().parse::<usize>().expect("Failed to parse input.");
    }

    pub fn go(self) -> f64 {
        let needles_per_thread = self.num_needles / self.num_threads as u64;
        let mut hits: u64 = 0;
        let mut misses: u64 = 0;
        let mut rng = rand::thread_rng();
        let pool = ThreadPool::new(Experiment::MAX_THREADS);
        let (tx, rx) = channel();

        // moving ownership of everything in the closure from the parent to all threads in threadpool.
        for _ in 0..self.num_threads {
            let cloned_sender = tx.clone();
            let mut small_rng = SmallRng::from_rng(&mut rng).unwrap();
            pool.execute(move || {
                for _ in 0..needles_per_thread{
                    cloned_sender.send(self.sim(&mut small_rng)).unwrap();
                }
            });
        }

        drop(tx);
        for result in rx{
            if result {
                    hits += 1;
            }else {
                misses += 1;
            }
        }

        // pool.join();
        debug_assert_eq!(hits + misses, self.num_needles);
        self.calculate(hits)
    }

    fn calculate(self, hits: u64) -> f64 {
        2.0 * self.needle_len * self.num_needles as f64/ (self.line_dist * hits as f64)
    }

    // fn sim(self) -> u8 {
    //     let mut rng = thread_rng();
    //     let y = rng.gen_range(0..self.line_dist);
    //     let theta = rng.gen_range(0..180);
    //     // let y: u8 = random();
    //     // let theta: u8 = random(0..180);
    //     let mut result = 0;
    //     if self.needle_len * theta.to_radians().sin()  + y > self.line_dist{
    //         result += 1;
    //     }
    //     return result;
    // }

    fn sim(self,  rng: &mut SmallRng) -> bool {
        let y = rng.gen_range(0.0..self.line_dist);
        let theta: f64 = rng.gen_range(0.0..180.0);
        // let y: u8 = random();
        // let theta: u8 = random(0..180);
        self.needle_len * theta.to_radians().sin() + y > self.line_dist
    }
}
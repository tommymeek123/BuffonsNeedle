use std::io::*;
use threadpool::ThreadPool;
use std::thread;
use rand::prelude::*;
use std::sync::mpsc::channel;

#[derive(Copy, Clone)]
pub struct Experiment {
    pub needle_len: u8,
    pub line_dist: u8,
    pub num_needles: u32,
    pub num_threads: usize,
}

impl Experiment {
    pub fn new() -> Self {
        Experiment {
            needle_len: 0,
            line_dist: 0,
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
        self.needle_len = buf.trim().parse::<u8>().expect("Failed to parse input.");

        // get line distance
        print!("Please enter the distance between the lines: ");
        stdout().flush().expect("Failed to flush stdout.");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read user input.");
        self.line_dist = buf.trim().parse::<u8>().expect("Failed to parse input.");

        // get the number of needles
        print!("Please enter the total number of needles: ");
        stdout().flush().expect("Failed to flush stdout.");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read user input.");
        self.num_needles = buf.trim().parse::<u32>().expect("Failed to parse input.");

        // get the number of threads
        print!("Please enter the number of threads: ");
        stdout().flush().expect("Failed to flush stdout.");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read user input.");
        self.num_threads = buf.trim().parse::<usize>().expect("Failed to parse input.");
    }

    pub fn go(self) {
        let pool = ThreadPool::new(self.num_threads);
        let (tx, rx) = channel();
        let cloned_sender = tx.clone();
        // moving ownership of everything in the closure from the parent to the all threads in threadpool.
        pool.execute(move || {
            cloned_sender.send(self.sim()).unwrap();
        });
        //pool.join();
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

    fn sim(self, rng: SmallRng) -> bool {
        let mut rng = thread_rng();
        let y = rng.gen_range(0..self.line_dist);
        let theta = rng.gen_range(0..180);
        // let y: u8 = random();
        // let theta: u8 = random(0..180);
        self.needle_len * theta.to_radians().sin() + y > self.line_dist
    }
}
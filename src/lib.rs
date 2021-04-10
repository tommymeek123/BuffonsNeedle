use std::io::*;
use threadpool::ThreadPool;
use std::thread;

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
    }

    fn sim() -> bool {
        // Generate a random y coordinate. Generate a random angle theta. Return true if 
        // self.needle_len * cos(theta) + y > self.line_dist.
        return true;
    }
}
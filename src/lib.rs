//! Buffon's Needle Experiment
//!
//! This module defines a struct called Experiment used to simulate the famous **Buffon's Needle**
//! problem. The problem is stated as follows: "Suppose we have a floor made of parallel strips of
//! wood, each the same width, and we drop a needle onto the floor. What is the probability that the
//! needle will lie across a line between two strips?"
//!
//! The answer the the problem, assuming that the needles are no longer than the width of the wooden
//! strips is (2*l) / (pi*w) where l is the length of the needles and w is the width of the wooden
//! strips. This simulation can be used to approximate the value of pi.
//!
//! Western Carolina University\
//! CS 370 Operating Systems\
//! Spring 2021
//!
//! **Authors:** Tommy Meek and Hannah Young

use rand::prelude::*;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

/// A struct for performing an experiment to find the value of pi.
#[derive(Copy, Clone)]
pub struct Experiment {
    pub needle_len: f64,
    pub line_dist: f64,
    pub num_needles: u64,
    pub num_threads: usize,
    needles_per_thread: u64,
}

impl Experiment {
    /// A constant to restrict the number of threads in the thread pool.
    pub const MAX_THREADS: usize = 200;

    /// A constructor for the Experiment struct.
    pub fn new() -> Self {
        Experiment {
            /// The length of each needle.
            needle_len: 0.0,

            /// The distance between the lines.
            line_dist: 0.0,

            /// The number of needles to drop.
            num_needles: 0,

            /// The number of threads to use in the experiment.
            num_threads: 0,

            /// How many needles each thread is responsible for dropping. (num_needles/num_threads)
            needles_per_thread: 0,
        }
    }

    /// Collects user input and sets all fields accordingly.
    pub fn input(&mut self) {
        self.get_needle_len();
        self.get_line_dist();
        self.get_num_needles();
        self.get_num_threads();

        // Calculate the number of needles to be dropped by each thread.
        self.needles_per_thread = self.num_needles / self.num_threads as u64;
    }

    /// Prompts the user for the length of the needles and sets the field accordingly.
    fn get_needle_len(&mut self) {
        while self.needle_len == 0.0 {
            print!("Please enter the length of the needles: ");
            stdout().flush().expect("Failed to flush stdout.");
            let mut buf = String::new();
            stdin().read_line(&mut buf).expect("Failed to read user input.");
            if let Ok(temp_len) = buf.trim().parse::<f64>() {
                if temp_len > 0.0 {
                    self.needle_len = temp_len;
                } else {
                    println!("Needle length should be positive.")
                }
            } else {
                println!("Failed to parse input.");
            }
        }
    }

    /// Prompts the user for the distance between the lines and sets the field accordingly.
    fn get_line_dist(&mut self) {
        while self.line_dist == 0.0 {
            print!("Please enter the distance between the lines: ");
            stdout().flush().expect("Failed to flush stdout.");
            let mut buf = String::new();
            stdin().read_line(&mut buf).expect("Failed to read user input.");
            if let Ok(temp_dist) = buf.trim().parse::<f64>() {
                if temp_dist > self.needle_len {
                    self.line_dist = temp_dist;
                } else {
                    println!("Line distance should be greater than needle length.")
                }
            } else {
                println!("Failed to parse input.");
            }
        }
    }

    /// Prompts the user for the number of needles and sets the field accordingly.
    fn get_num_needles(&mut self) {
        while self.num_needles == 0 {
            print!("Please enter the total number of needles: ");
            stdout().flush().expect("Failed to flush stdout.");
            let mut buf = String::new();
            stdin().read_line(&mut buf).expect("Failed to read user input.");
            if let Ok(temp_needles) = buf.trim().parse::<u64>() {
                if temp_needles > 0 {
                    self.num_needles = temp_needles;
                } else {
                    println!("Expected positive integer.");
                }
            } else {
                println!("Failed to parse input. Expected positive integer.");
            }
        }
    }

    /// Prompts the user for the number of threads and sets the field accordingly.
    fn get_num_threads(&mut self) {
        while self.num_threads == 0 {
            print!("Please enter the number of threads: ");
            stdout().flush().expect("Failed to flush stdout.");
            let mut buf = String::new();
            stdin().read_line(&mut buf).expect("Failed to read user input.");
            if let Ok(temp_threads) = buf.trim().parse::<usize>() {
                if 0 < temp_threads && temp_threads <= self.num_needles as usize {
                    self.num_threads = temp_threads;
                } else {
                    println!("Number of threads should be between 1 and the number of needles.");
                }
            } else {
                println!("Failed to parse input. Expected positive integer.");
            }
        }
    }

    /// This method activates the experiment.
    ///
    /// # Return value
    ///
    /// Returns an approximation of pi.
    pub fn go(self) -> f64 {
        let mut rng = rand::thread_rng();
        let pool = ThreadPool::new(Experiment::MAX_THREADS);
        let (sender, receiver) = channel();

        for _ in 0..self.num_threads {
            // Clones the sender so each thread can know where to send their results.
            let cloned_sender = sender.clone();

            // Create a lighter weight rng that is better for multi-threaded work.
            let mut small_rng = SmallRng::from_rng(&mut rng).expect("Failed to create RNG.");

            // Execute a single thread.
            pool.execute(move || {
                // Perform the simulation and send the result back to the main process.
                cloned_sender.send(self.sim(&mut small_rng)).expect("Error while passing message.");
            });
        }

        // Drops the original thread that was returned by channel().
        drop(sender);

        // Collects the result of each thread and sums them together before calculating pi.
        let mut hits: u64 = 0;
        for result in receiver {
            hits += result;
        }
        if hits == 0 {
            println!("No needles crossed the lines. Try dropping more needles.")
        }
        self.calculate(hits)
    }

    /// Uses the results of the experiment to approximate the value of pi.
    ///
    /// # Arguments
    ///
    /// * `hits` - The number of times the needle crossed the line in our simulation.
    ///
    /// # Return value
    ///
    /// An approximation of pi.
    fn calculate(self, hits: u64) -> f64 {
        2.0 * self.needle_len * self.num_needles as f64 / (self.line_dist * hits as f64)
    }

    /// Runs several simulations. Each thread in this program will execute this method once.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator used to generate two values. The first is the distance
    ///           the needle lands from the line below it. The second is the angle the needle will
    ///           be pointing.
    ///
    /// # Return value
    ///
    /// Returns the number of needles that ended up crossing a line.
    fn sim(self, rng: &mut SmallRng) -> u64 {
        let mut hits: u64 = 0;

        // Each iteration of this loop represents a single needle being dropped.
        for _ in 0..self.needles_per_thread {
            // If the parallel lines are visualized as being horizontal, this is the y coordinate.
            let y = rng.gen_range(0.0..self.line_dist);

            // The angle the needle is pointing.
            let theta: f64 = rng.gen_range(0.0..180.0);

            // Here, we assume the lower endpoint of the needle is below the line. We determine if
            // the other endpoint is above the line. If so, the simulation is considered a hit.
            if self.needle_len * theta.to_radians().sin() + y > self.line_dist {
                hits += 1;
            }
        }
        hits
    }
}

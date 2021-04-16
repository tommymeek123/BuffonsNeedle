//! Buffon's Needle
//! 
//! This is the driver for the Buffon's Needle simulation.
//! 
//! Western Carolina University\
//! CS 370 Operating Systems\
//! Spring 2021
//! 
//! **Authors:** Tommy Meek and Hannah Young

pub mod lib;
use crate::lib::Experiment;

/// The main function which conducts the experiment and prints the result to the console.
fn main() {
    let mut exp = Experiment::new();
    exp.input();
    let result = exp.go();
    const SEPARATOR: &str = "* * * * * * * * * * * * * * * * * * * *";
    println!("{}\nEstimated value of pi: {}\n{}", SEPARATOR, result, SEPARATOR);
}

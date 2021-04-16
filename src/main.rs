pub mod lib;
use crate::lib::Experiment;

fn main() {
    let mut exp = Experiment::new();
    exp.input();
    let result = exp.go();
    println!("Estimated value of pi: {}", result);
}

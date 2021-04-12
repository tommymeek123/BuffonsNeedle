pub mod lib;
use crate::lib::Experiment;

#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let mut exp = Experiment::new();
    exp.input();
    exp.go();
    // println!("needle length: {}, line width: {}, num needles: {}, num threads: {}", exp.needle_len, exp.line_dist, exp.num_needles, exp.num_threads);
}

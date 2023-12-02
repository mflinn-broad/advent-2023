mod day01;
mod day02;

pub fn run() {
    println!("Day 01 ***********");
    day01::run().expect("uh oh");
    println!("Day 02 ***********");
    day02::run().expect("oops");
}

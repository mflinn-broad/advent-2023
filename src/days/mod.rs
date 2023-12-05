mod day01;
mod day02;
mod day03;

pub fn run() {
    println!("Day 01 ***********");
    day01::run().expect("uh oh");
    println!("Day 02 ***********");
    day02::run().expect("oops");
    println!("Day 03 ***********");
    day03::run().expect("oh no");
}

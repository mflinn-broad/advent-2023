mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub fn run() {
    println!("Day 01 ***********");
    day01::run().expect("uh oh");
    println!("Day 02 ***********");
    day02::run().expect("oops");
    println!("Day 03 ***********");
    day03::run().expect("oh no");
    println!("Day 04 ***********");
    day04::run().expect("something is awry");
    println!("Day 05 ***********");
    day05::run().expect("yikes");
    println!("Day 06 ***********");
    day06::run().expect("womp womp");
    println!("Day 07 ***********");
    day07::run().expect("card sharks");
}

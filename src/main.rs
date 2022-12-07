mod day1;
mod day2;
mod day3;
mod day6;

use crate::{day1::day_one, day2::day_two, day3::day_three, day6::day_six};

fn main() {
    println!("Hello, world!");

    day_one::run();
    day_two::run();
    day_three::run();
    day_six::run()
}

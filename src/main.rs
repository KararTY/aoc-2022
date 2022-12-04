mod day1;
mod day2;
mod day3;

use crate::{day1::day_one, day2::day_two, day3::day_three};

fn main() {
    println!("Hello, world!");

    day_one::run();
    day_two::run();
		day_three::run();
}

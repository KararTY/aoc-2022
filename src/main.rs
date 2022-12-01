use std::fs;

fn main() {
    println!("Hello, world!");

    day_one();
}

fn day_one() {
    let file_contents: String = fs::read_to_string("./src/inputs/day1.txt")
        .expect("Could not find src/inputs/day1.txt file");

    let elves = file_contents.split("\r\n\r\n");

    let elves_calories: Vec<i32> = elves
        .map(|elf| {
            elf.split("\r\n")
                .fold(0, |accum, item| accum + item.parse::<i32>().unwrap())
                .to_owned()
        })
        .collect();

    // Is clone() necessary?
    let result = elves_calories
        .clone()
        .into_iter()
        .reduce(|high, item| if item > high { item } else { high })
        .unwrap();

    println!("Day 1 Part 1:\n{}", result);

    day_one_part_two(&elves_calories);
}

fn day_one_part_two(elves_calories: &Vec<i32>) {
    let mut largest_numbers: Vec<i32> = [0, 0, 0].to_vec();

    for calories in elves_calories {
        for i in 0..largest_numbers.len() {
            if calories > &largest_numbers[i] {
                largest_numbers[i] = *calories;
                break;
            }
        }
    }

    let result: i32 = largest_numbers.into_iter().sum();

    println!("Day 1 Part 2:\n{:?}", result);
}

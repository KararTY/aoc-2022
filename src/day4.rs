use std::{fs, str::Lines};

fn section_into_int_vec(section: &str) -> Vec<i32> {
    let mut int_vec: Vec<i32> = Vec::new();

    let first_ranges: Vec<i32> = section
        .split("-")
        .map(|string| string.parse::<i32>().unwrap())
        .collect();

    for range in first_ranges[0]..(first_ranges[1] + 1) {
        int_vec.push(range);
    }

    int_vec
}

pub fn run() {
    let file_contents: String = fs::read_to_string("./src/inputs/day4.txt")
        .expect("Could not find src/inputs/day4.txt file");

    let lines = file_contents.lines();

    let mut result: i32 = 0;

    for line in lines.clone() {
        let sections: Vec<&str> = line.split(",").collect();

        let (first_sections, second_sections) = (sections[0], sections[1]);

        let first_numbers: Vec<i32> = section_into_int_vec(first_sections);
        let second_numbers: Vec<i32> = section_into_int_vec(second_sections);

        let mut duplicates: Vec<i32> = Vec::new();

        for f_number in first_numbers.clone() {
            if second_numbers.contains(&f_number) {
                duplicates.push(f_number);
            }
        }

        if duplicates.len() == second_numbers.len() {
            result += 1;
            continue;
        }

        let mut second_duplicates: Vec<i32> = Vec::new();

        for s_number in second_numbers {
            if first_numbers.contains(&s_number) {
                second_duplicates.push(s_number);
            }
        }

        if second_duplicates.len() == first_numbers.len() {
            result += 1;
        }
    }

    println!("Day 4 Part One:\n{}", result);

    part_two(lines);
}

fn part_two(lines: Lines) {
    let mut result: i32 = 0;

    for line in lines.clone() {
        let sections: Vec<&str> = line.split(",").collect();

        let (first_sections, second_sections) = (sections[0], sections[1]);

        let first_numbers: Vec<i32> = section_into_int_vec(first_sections);
        let second_numbers: Vec<i32> = section_into_int_vec(second_sections);

        let mut cont = false;
        for f_number in first_numbers.clone() {
          if second_numbers.contains(&f_number) {
              result += 1;
              cont = true;
              break
          }
        }

        if cont {
          continue;
        }
    }

    println!("Day 4 Part Two:\n{}", result);
}

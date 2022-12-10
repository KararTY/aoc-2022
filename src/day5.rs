use std::{fs, str::Lines};

pub fn run() {
    let file_contents: String = fs::read_to_string("./src/inputs/day5.txt")
        .expect("Could not find src/inputs/day5.txt file");

    let lines = file_contents.lines();

    let width_line = lines
        .clone()
        .enumerate()
        .find(|(_, line)| !line.contains("["))
        .unwrap();

    let width = width_line
        .1
        .trim()
        .split(" ")
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let mut vec: Vec<Vec<char>> = Vec::new();
    for _ in 0..width {
        vec.push(Vec::new());
    }

    let blacklist = vec!['[', ']', ' '];

    for index in 0..width_line.0 {
        let line = lines.clone().nth(index).unwrap();

        let characters = line.chars().enumerate();

        let mut count = 0;
        for (index, character) in characters {
            if index % 4 == 0 {
                count += 1;
            }

            if !blacklist.contains(&character) {
                vec.get_mut(count - 1).unwrap().push(character);
            }
        }
    }

    let mut instructions = lines.clone();
    instructions.nth((width_line.0 + 1) as usize);

    let mut part_one_vector = vec.clone();
    for instruction in instructions.clone() {
        let amount: usize = instruction.split(" ").nth(1).unwrap().parse().unwrap();
        let from: usize = instruction.split(" ").nth(3).unwrap().parse().unwrap();
        let to: usize = instruction.split(" ").nth(5).unwrap().parse().unwrap();

        let row = part_one_vector.get_mut(from - 1).unwrap();

        let mut cargos: Vec<char> = row.drain(0..amount).collect();

        let to_row = part_one_vector.get_mut(to - 1).unwrap();

        to_row.reverse();

        to_row.append(&mut cargos);

        to_row.reverse();
    }

    println!(
        "Day 5 Part One:\n{}",
        part_one_vector
            .into_iter()
            .map(|char_vec| char_vec.first().unwrap().to_string())
            .collect::<Vec<String>>()
            .join("")
    );

    part_two(vec, instructions);
}

fn part_two(vec: Vec<Vec<char>>, instructions: Lines) {
    let mut part_two_vector = vec.clone();

    for instruction in instructions.clone() {
        let amount: usize = instruction.split(" ").nth(1).unwrap().parse().unwrap();
        let from: usize = instruction.split(" ").nth(3).unwrap().parse().unwrap();
        let to: usize = instruction.split(" ").nth(5).unwrap().parse().unwrap();

        let row = part_two_vector.get_mut(from - 1 as usize).unwrap();

        let mut cargos: Vec<char> = row.drain(0..amount).collect();
        cargos.reverse();

        let to_row = part_two_vector.get_mut(to - 1).unwrap();

        to_row.reverse();

        to_row.append(&mut cargos);

        to_row.reverse();
    }

    println!(
        "Day 5 Part Two:\n{}",
        part_two_vector
            .into_iter()
            .map(|char_vec| char_vec.first().unwrap().to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}


use std::fs;

pub fn run() {
    let file_contents: String = fs::read_to_string("./src/inputs/day6.txt")
        .expect("Could not find src/inputs/day6.txt file");

    let mut last_characters: Vec<(usize, char)> = Vec::new();

    for (index, character) in file_contents.chars().enumerate() {
        let found = last_characters
            .iter()
            .enumerate()
            .find(|(_, (_, existing_character))| character == *existing_character);

        match found {
            Some(found) => {
                last_characters.drain(0..found.0 + 1);
                last_characters.push((index + 1, character));
            }
            None => {
                last_characters.push((index + 1, character));

                if last_characters.len() >= 4 {
                    break;
                }
            }
        }
    }

    println!("Day 6 Part One:\n{}", last_characters.last().unwrap().0);

    part_two(file_contents);
}

fn part_two(file_contents: String) {
    let mut last_characters: Vec<(usize, char)> = Vec::new();

    // TODO: Dry this.
    for (index, character) in file_contents.chars().enumerate() {
        let found = last_characters
            .iter()
            .enumerate()
            .find(|(_, (_, existing_character))| character == *existing_character);

        match found {
            Some(found) => {
                last_characters.drain(0..found.0 + 1);
                last_characters.push((index + 1, character));
            }
            None => {
                last_characters.push((index + 1, character));

                if last_characters.len() >= 14 {
                    break;
                }
            }
        }
    }

    println!("Day 6 Part Two:\n{}", last_characters.last().unwrap().0);
}

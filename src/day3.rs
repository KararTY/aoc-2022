pub mod day_three {
    use std::{
        collections::{BTreeMap, BTreeSet},
        fs,
        str::Lines,
    };

    pub fn run() {
        let file_contents: String = fs::read_to_string("./src/inputs/day3.txt")
            .expect("Could not find src/inputs/day3.txt file");

        let lines = file_contents.lines();

        let mut result: i32 = 0;

        for line in lines.clone() {
            let mut items: BTreeSet<char> = BTreeSet::new();
            let mut duplicate_item: Option<char> = None;

            let (first_compartment, second_compartment) = line.split_at(line.len() / 2);

            for character in first_compartment.chars() {
                items.insert(character);
            }

            for character in second_compartment.chars() {
                if items.contains(&character) {
                    duplicate_item = Some(character);
                    break;
                }
            }

            if let Some(duplicate_item) = duplicate_item {
                result += calculate_character_priority(duplicate_item);
            }
        }
        println!("Day 3 Part One:\n{}", result);

        part_two(lines);
    }

    fn part_two(lines: Lines) {
        let mut result: i32 = 0;

        let mut groups = Vec::new();

        for (index, line) in lines.enumerate() {
            if index % 3 == 0 {
                let mut vec = Vec::new();
                vec.push(line);

                groups.push(vec);
            } else {
                groups.last_mut().unwrap().push(line);
            }
        }

        for group_lines in groups {
            let mut count_of_unique_chars: BTreeMap<char, i32> = BTreeMap::new();

            for line in group_lines {
                let mut items: BTreeSet<char> = BTreeSet::new();

                for character in line.chars() {
                    items.insert(character);
                }

                for character in items {
                    let entry = count_of_unique_chars.entry(character).or_default();
                    (*entry) += 1;
                }
            }

            let unique_char = count_of_unique_chars.iter().find(|(_, &hits)| hits == 3);

            if let Some(unique_char) = unique_char {
                result += calculate_character_priority(unique_char.0.clone());
            }
        }

        println!("Day 3 Part Two:\n{:?}", result);
    }

    fn calculate_character_priority(character: char) -> i32 {
        if character.is_uppercase() {
            return character as i32 - 38;
        } else {
            return character as i32 - 96;
        }
    }
}

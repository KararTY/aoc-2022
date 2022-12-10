use std::{fs, str::Lines};

// https://www.reddit.com/r/rust/comments/vk0quc/how_to_best_implement_a_static_lookup_map/
const fn decode_shape_score(x: &char) -> i32 {
    match x {
        'A' => 1, // Rock
        'B' => 2, // Paper
        'C' => 3, // Scissor

        'X' => 1, // Rock
        'Y' => 2, // Paper
        'Z' => 3, // Scissor
        _ => 0,
    }
}

const fn decode_shape_result_won(enemy_character: &char, strategy_character: &char) -> bool {
    match (enemy_character, strategy_character) {
        ('A', 'Y') => true,
        ('B', 'Z') => true,
        ('C', 'X') => true,
        _ => false,
    }
}

const fn decode_shape_response(enemy_character: char, strategy_character: char) -> Option<char> {
    match (enemy_character, strategy_character) {
        // Loss
        ('A', 'X') => Some('Z'),
        ('B', 'X') => Some('X'),
        ('C', 'X') => Some('Y'),

        // Win
        ('A', 'Z') => Some('Y'),
        ('B', 'Z') => Some('Z'),
        ('C', 'Z') => Some('X'),

        // Draw
        (_, 'Y') => Some(enemy_character),
        _ => None,
    }
}

pub fn run() {
    let file_contents: String = fs::read_to_string("./src/inputs/day2.txt")
        .expect("Could not find src/inputs/day2.txt file");

    let lines = file_contents.lines();

    let mut result: i32 = 0;

    lines.clone().for_each(|line| {
        let characters: Vec<char> = line
            .trim()
            .split(" ")
            .map(|string| string.trim().chars().nth(0).unwrap())
            .collect();

        let (enemy_character, strategy_character) = (characters[0], characters[1]);

        let enemy_score = decode_shape_score(&enemy_character);
        let my_score = decode_shape_score(&strategy_character);

        // Add my shape to result.
        result += my_score;

        // Add draw to result.
        if my_score == enemy_score {
            result += 3;
        }

        // Did my strategy win?
        if decode_shape_result_won(&enemy_character, &strategy_character) {
            result += 6;
        }
    });

    println!("Day 2 Part One:\n{}", result);

    part_two(lines);
}

fn part_two(lines: Lines) {
    let mut result: i32 = 0;

    lines.for_each(|line| {
        let characters: Vec<char> = line
            .trim()
            .split(" ")
            .map(|string| string.trim().chars().nth(0).unwrap())
            .collect();

        let (enemy_character, strategy_character) = (characters[0], characters[1]);

        let enemy_score = decode_shape_score(&enemy_character);

        let my_shape_response = decode_shape_response(enemy_character, strategy_character);

        if my_shape_response.is_some() {
            let my_actual_shape = &my_shape_response.unwrap();
            let my_score = decode_shape_score(my_actual_shape);

            // Add my shape result to result.
            result += my_score;

            // Add draw to result.
            if my_score == enemy_score {
                result += 3;
            }

            // Did my strategy win?
            if decode_shape_result_won(&enemy_character, my_actual_shape) {
                result += 6;
            }
        }
    });

    println!("Day 2 Part Two:\n{}", result);
}

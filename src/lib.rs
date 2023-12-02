use crate::game::CubesPulled;

mod game;
mod string_utils;

pub fn solve_day_1_part_1(lines: Vec<String>) -> i32 {
    lines
        .iter()
        .map(|line| string_utils::get_number_from_string(line, false))
        .sum()
}

pub fn solve_day_1_part_2(lines: Vec<String>) -> i32 {
    lines
        .iter()
        .map(|line| string_utils::get_number_from_string(line, true))
        .sum()
}

pub fn solve_day_2_part_1(lines: Vec<String>) -> i32 {
    const MAX_BLUE: i32 = 14;
    const MAX_GREEN: i32 = 13;
    const MAX_RED: i32 = 12;

    lines
        .iter()
        .map(|line| game::map_input_to_cubes_puled(line))
        .map(|collection| CubesPulled::collect(collection))
        .filter(|item| item.red <= MAX_RED && item.green <= MAX_GREEN && item.blue <= MAX_BLUE)
        .map(|item| item.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_day_1_part_1_with_example_input() {
        let input = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ];
        let expected_result = 142;

        let result = solve_day_1_part_1(input);
        assert_eq!(expected_result, result)
    }

    #[test]
    fn solve_day_1_part_2_with_example_input() {
        let input = vec![
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        ];
        let expected_result = 281;

        let result = solve_day_1_part_2(input);
        assert_eq!(expected_result, result)
    }

    #[test]
    fn solve_day_2_part_1_with_example_input() {
        let input = vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            ),
            String::from(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            ),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ];

        let expected_result = 8;
        let result = solve_day_2_part_1(input);
        assert_eq!(expected_result, result)
    }

    #[test]
    fn solve_day_1_part_1_with_challenge_input() {
        let lines: Vec<String> = include_str!("../inputs/day_1.txt")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let expected_result = 56397;
        let result = solve_day_1_part_1(lines);
        assert_eq!(expected_result, result)
    }

    #[test]
    fn solve_day_1_part_2_with_challenge_input() {
        let lines: Vec<String> = include_str!("../inputs/day_1.txt")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let expected_result = 55701;
        let result = solve_day_1_part_2(lines);
        assert_eq!(expected_result, result)
    }
}

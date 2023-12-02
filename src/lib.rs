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

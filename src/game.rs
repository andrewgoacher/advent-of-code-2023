use std::collections::HashMap;

use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct CubesPulled {
    pub id: i32,
    pub blue: i32,
    pub green: i32,
    pub red: i32,
}

fn get_game_and_input(input: &str) -> (i32, &str) {
    let mut parts = input.split(":");
    let game_part = parts.next().unwrap();
    let rest = parts.next().unwrap();
    let id = game_part.replace("Game ", "").parse::<i32>().unwrap();

    (id, rest.trim())
}

fn get_individual_runs(input: &str) -> Vec<String> {
    let parts = input.split(";");
    parts
        .into_iter()
        .map(|str| str.trim().to_string())
        .collect()
}

pub fn map_input_to_cubes_puled(input: &str) -> Vec<CubesPulled> {
    let (id, rest) = get_game_and_input(input);
    let parts = get_individual_runs(rest);

    parts
        .iter()
        .map(|part| CubesPulled::from_string(id, part))
        .collect()
}

impl CubesPulled {
    fn new(id: i32, r: i32, g: i32, b: i32) -> Self {
        Self {
            id,
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn collect(cubes: Vec<CubesPulled>) -> Self {
        let (r, g, b) = cubes.iter().fold((0, 0, 0), |aggregate, item| {
            println!("Collecting: {:?}", item);
            return (
                if aggregate.0 > item.red {
                    aggregate.0
                } else {
                    item.red
                },
                if aggregate.1 > item.green {
                    aggregate.1
                } else {
                    item.green
                },
                if aggregate.2 > item.blue {
                    aggregate.2
                } else {
                    item.blue
                },
            );
        });

        Self {
            id: cubes[0].id,
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn from_string(id: i32, input: &str) -> Self {
        let pattern = r"(\d+) (\w+)";
        let re = Regex::new(pattern).unwrap();
        let map: HashMap<String, i32> = re
            .captures_iter(input)
            .map(|m| (m[2].to_string(), m[1].parse::<i32>().unwrap_or(0)))
            .collect();

        Self::new(
            id,
            map.get("red").copied().unwrap_or(0),
            map.get("green").copied().unwrap_or(0),
            map.get("blue").copied().unwrap_or(0),
        )
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;

    macro_rules! get_game_and_input_has_correct_id {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let number = $value;
            let input = format!("Game {}: 3 blue, 4 red", number);
            let (id, _) = get_game_and_input(&input);
            assert_eq!(number, id)
        }
    )*
    }
    }

    get_game_and_input_has_correct_id! {
        test_1: (1),
        test_2: (9),
        test_3: (50),
    }

    #[test]
    fn get_game_and_input_gets_correct_id() {
        let input = "Game 1: 3 blue, 4 red";
        let (id, _) = get_game_and_input(input);

        assert_eq!(1, id)
    }

    #[test]
    fn get_game_and_input_gets_correct_remainder() {
        let input = "Game 1: 3 blue, 4 red";
        let (_, rest) = get_game_and_input(input);

        assert_eq!("3 blue, 4 red", rest)
    }

    #[test]
    fn get_individual_runs_single_input_returns_correct_input() {
        let input = "3 blue, 4 red";
        let collection = get_individual_runs(input);

        let expected = vec![String::from("3 blue, 4 red")];
        assert_eq!(expected, collection)
    }

    #[test]
    fn get_individual_runs_multiple_input_returns_correct_input() {
        let input = "3 blue, 4 red ; 1 green, 2 red";
        let collection = get_individual_runs(input);

        let expected = vec![
            String::from("3 blue, 4 red"),
            String::from("1 green, 2 red"),
        ];
        assert_eq!(expected, collection)
    }

    #[test]
    fn cubes_pulled_from_string_single_input_returns_correct_cube() {
        let input = "3 blue, 4 red, 1 green";
        let id = 5;
        let expected = CubesPulled::new(id, 4, 1, 3);

        let collection = CubesPulled::from_string(id, input);

        assert_eq!(expected, collection)
    }

    #[test]
    fn map_input_to_cubes_puled_input_string_produces_correct_output() {
        let input = "Game 5: 13 blue, 4 red, 1 green; 1 blue, 20 red";
        let id = 5;
        let expected_collection = vec![
            CubesPulled::new(id, 4, 1, 13),
            CubesPulled::new(id, 20, 0, 1),
        ];

        let collection = map_input_to_cubes_puled(input);

        assert_eq!(expected_collection, collection)
    }
}

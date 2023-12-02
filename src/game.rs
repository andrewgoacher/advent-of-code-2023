pub struct CubesPulled {
    id: i32,
    blue: i32,
    green: i32,
    red: i32,
}

fn get_game_and_input(input: &str) -> (i32, &str) {
    let mut parts = input.split(":");
    let game_part = parts.next().unwrap();
    let rest = parts.next().unwrap();
    let id = game_part.replace("Game ", "").parse::<i32>().unwrap();

    (id, rest.trim())
}

fn get_individual_runs(input: &str) -> Vec<String> {
    let mut parts = input.split(";");
    parts
        .into_iter()
        .map(|str| str.trim().to_string())
        .collect()
}

// pub fn map_input_to_cubes_puled(input: &str) -> Vec<CubesPulled> {
//    let (id, rest) = get_game_and_input(input);
//     let mut parts = rest.split(";");
//
//     parts
//         .map(|part| CubesPulled::from_string(id, part))
//         .collect()
// }
//
// impl CubesPulled {
//     pub fn from_string(id: i32, input: &str) -> Self {
//         let pattern = r"(\d+) (\w+)";
//         let re = Regex::new(pattern).unwrap();
//         let map: HashMap<String, i32> = re.captures_iter(input)
//             .map(|m| (m[2].to_string().clone(), m[1].parse::<i32>()))
//             .collect();
//
//         Self {
//             id,
//             blue: map.get("blue").unwrap_or(*0),
//             green: map.get("green").unwrap_or(*0),
//             red: map.get("red").unwrap_or(*0)
//         }
//     }
// }

#[cfg(test)]
mod game_tests {
    use super::*;

    #[test]
    fn get_game_and_input_gets_correct_id() {
        let input = "Game 1: 3 blue, 4 red";
        let (id, rest) = get_game_and_input(input);

        assert_eq!(1, id)
    }

    #[test]
    fn get_game_and_input_gets_correct_remainder() {
        let input = "Game 1: 3 blue, 4 red";
        let (id, rest) = get_game_and_input(input);

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
}

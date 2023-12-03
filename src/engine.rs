#[derive(Debug, PartialEq)]
pub struct Part {
    pub number: u32,
    component: Option<char>,
}

pub fn process_input(input: Vec<String>) -> Vec<Part> {
    let mut parts = vec![];

    for i in 0..input.len() {
        let line = input.get(i).expect("Invalid: Should have line here");
        let chars: Vec<char> = line.chars().collect();

        let chars_above = if i == 0 {
            vec![]
        } else {
            input.get(i - 1).map_or(vec![], |s| s.chars().collect())
        };

        let chars_below = input.get(i + 1).map_or(vec![], |s| s.chars().collect());

        let mut num = 0u32;
        let mut start_index = 0;

        for j in 0..line.len() {
            let current_char = chars.get(j).expect("Should always get a character here");
            if current_char.is_numeric() {
                let digit: u32 = current_char.to_digit(10).expect("Should be numerical");
                if num == 0 {
                    start_index = j;
                }
                num = (num * 10) + digit;
            } else if is_symbol(current_char) {
                if num > 0 {
                    parts.push(Part {
                        number: num,
                        component: Some(current_char.clone()),
                    });
                    num = 0;
                    start_index = 0;
                }
            } else {
                if num > 0 && j > 0 {
                    let component = check_surroundings(
                        chars_above.clone(),
                        chars_below.clone(),
                        chars.clone(),
                        start_index,
                        j - 1,
                    );
                    parts.push(Part {
                        number: num,
                        component,
                    });
                    num = 0;
                }
            }
        }
    }

    parts
        .into_iter()
        .filter(|part| part.component.is_some())
        .collect()
}

fn check_surroundings(
    above: Vec<char>,
    below: Vec<char>,
    current: Vec<char>,
    start_index: usize,
    end_index: usize,
) -> Option<char> {
    let start_range = if start_index == 0 { 0 } else { start_index - 1 };

    let end_range = end_index + 1;

    let mut possibilities = vec![current.get(start_range), current.get(end_range)];

    for i in start_range..(end_range + 1) {
        possibilities.push(above.get(i));
        possibilities.push(below.get(i));
    }

    let possibilities: Vec<char> = possibilities
        .iter()
        .filter_map(|c| c.map(|c| c.clone()))
        .filter(|c| is_symbol(c))
        .collect();

    match possibilities.get(0) {
        Some(c) => Some(c.clone()),
        None => None,
    }
}

fn is_symbol(input: &char) -> bool {
    match input {
        '.' => false,
        x if x.is_alphanumeric() => false,
        _ => true,
    }
}

#[cfg(test)]
mod engine_tests {
    use super::*;

    macro_rules! is_symbol_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    let result = is_symbol(&input);
                    assert_eq!(expected, result)
                }
            )*
        }
    }

    is_symbol_tests! {
        is_period_symbol: ('.', false),
        is_letter_symbol: ('a', false),
        is_hash_symbol: ('#', true),
        is_asterisk_symbol: ('*', true),
    }

    #[test]
    fn collect_parts_single_line_no_parts_returns_empty_collection() {
        let input = "........";
        let actual = process_input(vec![String::from(input)]);
        let expected: Vec<Part> = vec![];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_single_line_with_number_no_parts_returns_empty_collection() {
        let input = "123.....";
        let actual = process_input(vec![String::from(input)]);
        let expected: Vec<Part> = vec![];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_single_line_with_number_and_symbol_returns_single_item() {
        let input = "123*.....";
        let actual = process_input(vec![String::from(input)]);
        let expected = vec![Part {
            number: 123,
            component: Some('*'),
        }];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_single_line_with_symbol_then_number_returns_single_item() {
        let input = "*123......";
        let actual = process_input(vec![String::from(input)]);
        let expected = vec![Part {
            number: 123,
            component: Some('*'),
        }];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_single_line_multiple_parts_returns_items() {
        let input = "*123..234#";
        let actual = process_input(vec![String::from(input)]);

        let expected = vec![
            Part {
                number: 123,
                component: Some('*'),
            },
            Part {
                number: 234,
                component: Some('#'),
            },
        ];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_single_line_with_symbol_no_nuber_returns_no_items() {
        let input = "...*....";
        let actual = process_input(vec![String::from(input)]);

        let expected: Vec<Part> = vec![];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_multiple_lines_single_part_returns_item() {
        let input = vec![
            String::from("........"),
            String::from("..123*.."),
            String::from("........"),
        ];

        let actual = process_input(input);

        let expected = vec![Part {
            number: 123,
            component: Some('*'),
        }];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_multiple_lines_multiple_single_line_parts_returns_items() {
        let input = vec![
            String::from("#334...."),
            String::from("..123*.."),
            String::from("....456#"),
        ];

        let actual = process_input(input);

        let expected = vec![
            Part {
                number: 334,
                component: Some('#'),
            },
            Part {
                number: 123,
                component: Some('*'),
            },
            Part {
                number: 456,
                component: Some('#'),
            },
        ];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_multiple_lines_with_symbol_and_number_not_touching_returns_empty() {
        let input = vec![
            String::from(".......*"),
            String::from("..123..."),
            String::from("........"),
        ];

        let actual = process_input(input);

        let expected: Vec<Part> = vec![];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_multiple_lines_single_item_touching_above_returns_item() {
        let input = vec![
            String::from("..*....."),
            String::from("..123..."),
            String::from("........"),
        ];

        let actual = process_input(input);

        let expected = vec![Part {
            number: 123,
            component: Some('*'),
        }];

        assert_eq!(expected, actual)
    }
}

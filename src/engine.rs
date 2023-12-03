use crate::engine::Component::Gear;

#[derive(Debug, PartialEq)]
pub enum Component {
    Gear,
    Component(char),
}

#[derive(Debug, PartialEq)]
pub struct Part {
    pub number: u32,
    component: Component,
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
            } else {
                if num > 0 {
                    let end_index = if j == 0 { 0 } else { j - 1 };
                    let components = check_surroundings(
                        chars_above.clone(),
                        chars_below.clone(),
                        chars.clone(),
                        start_index,
                        end_index,
                    );
                    for component in components {
                        parts.push(Part {
                            number: num,
                            component: get_component(component),
                        })
                    }
                    num = 0;
                }
            }
        }
        if num > 0 {
            let j = line.len() - 1;
            let end_index = if j == 0 { 0 } else { j - 1 };
            let components = check_surroundings(
                chars_above.clone(),
                chars_below.clone(),
                chars.clone(),
                start_index,
                end_index,
            );
            for component in components {
                parts.push(Part {
                    number: num,
                    component: get_component(component),
                })
            }
        }
        num = 0;
    }

    parts
}

fn check_surroundings(
    above: Vec<char>,
    below: Vec<char>,
    current: Vec<char>,
    start_index: usize,
    end_index: usize,
) -> Vec<char> {
    let start_range = if start_index == 0 { 0 } else { start_index - 1 };

    let end_range = end_index + 1;

    let mut possibilities = vec![current.get(start_range), current.get(end_range)];

    for i in start_range..(end_range + 1) {
        possibilities.push(above.get(i));
        possibilities.push(below.get(i));
    }

    possibilities
        .iter()
        .filter_map(|c| c.map(|c| c.clone()))
        .filter(|c| is_symbol(c))
        .collect()
}

fn get_component(input: char) -> Component {
    match input {
        '*' => Gear,
        _ => Component::Component(input),
    }
}

fn is_symbol(input: &char) -> bool {
    match input {
        '.' => false,
        '\r' => false,
        '\n' => false,
        x if x.is_numeric() => false,
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
        is_letter_symbol: ('a', true),
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
            component: Gear,
        }];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_single_line_with_symbol_then_number_returns_single_item() {
        let input = "*123......";
        let actual = process_input(vec![String::from(input)]);
        let expected = vec![Part {
            number: 123,
            component: Gear,
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
                component: Gear,
            },
            Part {
                number: 234,
                component: Component::Component('#'),
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
            component: Gear,
        }];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_multiple_lines_single_part_returns_item_2() {
        let input = vec![
            String::from("........"),
            String::from("....123#"),
            String::from("........"),
        ];

        let actual = process_input(input);

        let expected = vec![Part {
            number: 123,
            component: Component::Component('#'),
        }];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_multiple_lines_single_part_returns_item_3() {
        let input = vec![
            String::from("........"),
            String::from(".....123"),
            String::from(".......#"),
        ];

        let actual = process_input(input);

        let expected = vec![Part {
            number: 123,
            component: Component::Component('#'),
        }];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_multiple_lines_single_part_returns_item_4() {
        let input = vec![
            String::from(".......#"),
            String::from(".....123"),
            String::from("........"),
        ];

        let actual = process_input(input);

        let expected = vec![Part {
            number: 123,
            component: Component::Component('#'),
        }];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_multiple_lines_multiple_single_line_parts_returns_items() {
        let input = vec![
            String::from("#334....\r\n"),
            String::from("..123*..\r\n"),
            String::from("...#456.\r\n"),
            String::from("......78\r\n"),
        ];

        let actual = process_input(input);

        let expected = vec![
            Part {
                number: 334,
                component: Component::Component('#'),
            },
            Part {
                number: 123,
                component: Gear,
            },
            Part {
                number: 123,
                component: Component::Component('#'),
            },
            Part {
                number: 456,
                component: Component::Component('#'),
            },
            Part {
                number: 456,
                component: Gear,
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
            component: Gear,
        }];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_multiple_lines_additional_test_1() {
        let input = vec![
            String::from("........"),
            String::from(".24$-4.."),
            String::from("......*."),
        ];

        let actual = process_input(input);

        let expected = vec![
            Part {
                number: 24,
                component: Component::Component('$'),
            },
            Part {
                number: 4,
                component: Component::Component('-'),
            },
            Part {
                number: 4,
                component: Gear,
            },
        ];

        assert_eq!(expected, actual)
    }
}

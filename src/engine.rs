use crate::engine::Component::Gear;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Component {
    Gear(usize),
    Component(char, usize),
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
                        i,
                        line.len(),
                        start_index,
                        end_index,
                    );
                    for component in components {
                        parts.push(Part {
                            number: num,
                            component,
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
                i,
                line.len(),
                start_index,
                end_index,
            );
            for component in components {
                parts.push(Part {
                    number: num,
                    component,
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
    row_index: usize,
    len: usize,
    start_index: usize,
    end_index: usize,
) -> Vec<Component> {
    let start_range = if start_index == 0 { 0 } else { start_index - 1 };

    let end_range = end_index + 1;

    let mut possibilities: Vec<Component> = vec![];

    if let Some(start) = try_create_component(current.get(start_range), row_index, start_range, len)
    {
        possibilities.push(start);
    }

    if let Some(end) = try_create_component(current.get(end_range), row_index, end_range, len) {
        possibilities.push(end);
    }

    for i in start_range..(end_range + 1) {
        if row_index > 0 {
            if let Some(above) = try_create_component(above.get(i), row_index - 1, i, len) {
                possibilities.push(above);
            }
        }

        if let Some(below) = try_create_component(below.get(i), row_index + 1, i, len) {
            possibilities.push(below);
        }
    }

    possibilities
}

fn try_create_component(
    c: Option<&char>,
    row_index: usize,
    jpos: usize,
    len: usize,
) -> Option<Component> {
    match c {
        None => None,
        Some(c) => {
            let component_index = calculate_component_index(row_index, len, jpos);
            match is_symbol(c) {
                false => None,
                true => Some(get_component(c.clone(), component_index)),
            }
        }
    }
}

fn calculate_component_index(row_index: usize, len: usize, jpos: usize) -> usize {
    (row_index * len) + jpos
}

fn get_component(input: char, component_index: usize) -> Component {
    match input {
        '*' => Gear(component_index),
        _ => Component::Component(input.clone(), component_index),
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
            component: Gear(3),
        }];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_single_line_with_symbol_then_number_returns_single_item() {
        let input = "*123......";
        let actual = process_input(vec![String::from(input)]);
        let expected = vec![Part {
            number: 123,
            component: Gear(0),
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
                component: Gear(0),
            },
            Part {
                number: 234,
                component: Component::Component('#', 9),
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
            component: Gear(13),
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
            component: Component::Component('#', 15),
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
            component: Component::Component('#', 23),
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
            component: Component::Component('#', 7),
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
                component: Component::Component('#', 0),
            },
            Part {
                number: 123,
                component: Gear(15),
            },
            Part {
                number: 123,
                component: Component::Component('#', 23),
            },
            Part {
                number: 456,
                component: Component::Component('#', 23),
            },
            Part {
                number: 456,
                component: Gear(15),
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
            component: Gear(2),
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
                component: Component::Component('$', 11),
            },
            Part {
                number: 4,
                component: Component::Component('-', 12),
            },
            Part {
                number: 4,
                component: Gear(22),
            },
        ];

        assert_eq!(expected, actual)
    }
}

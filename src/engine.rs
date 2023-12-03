#[derive(Debug, PartialEq)]
struct Part {
    number: u32,
    component: char,
}

fn process_input(input: Vec<String>) -> Vec<Part> {
    let mut parts = vec![];
    let mut val = 0;
    let mut part_char = '.';

    let input_length = input.len();

    for i in 0..input_length {
        let line = input.get(i).expect("Should have value");
        let chars: Vec<char> = line.chars().collect();

        let chars_above = if i == 0 {
            vec![]
        } else {
            input.get(i - 1).map_or(vec![], |s| s.chars().collect())
        };

        let chars_below = input.get(i + 1).map_or(vec![], |s| s.chars().collect());

        for j in 0..line.len() {
            let current_char = chars.get(j).expect("should have char");
            match current_char {
                '.' => {
                    if val > 0 && part_char != '.' {
                        parts.push(Part {
                            number: val,
                            component: part_char,
                        });
                    }
                    val = 0;

                    let mut possibilities = vec![
                        chars_above.get(j),
                        chars_above.get(j + 1),
                        chars_below.get(j),
                        chars_below.get(j + 1),
                    ];

                    if j > 0 {
                        possibilities.push(chars_above.get(j - 1));
                        possibilities.push(chars_below.get(j - 1));
                    }

                    let possibilities: Vec<&char> = possibilities
                        .into_iter()
                        .filter_map(|p| p.map(|p| p))
                        .collect();

                    if possibilities.len() == 0 {
                        part_char = '.';
                    }
                }
                x if is_symbol(x) => {
                    if val > 0 {
                        parts.push(Part {
                            number: val,
                            component: current_char.clone(),
                        });
                        val = 0;
                        part_char = '.';
                    } else {
                        part_char = current_char.clone();
                    }
                }
                x if x.is_numeric() => {
                    let num = x.to_digit(10).expect("Should be numberic");
                    val = (val * 10) + num;
                }
                _ => {}
            };
        }
    }

    parts
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
            component: '*',
        }];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_single_line_with_symbol_then_number_returns_single_item() {
        let input = "*123......";
        let actual = process_input(vec![String::from(input)]);
        let expected = vec![Part {
            number: 123,
            component: '*',
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
                component: '*',
            },
            Part {
                number: 234,
                component: '#',
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
            component: '*',
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
                component: '#',
            },
            Part {
                number: 123,
                component: '*',
            },
            Part {
                number: 456,
                component: '#',
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
            component: '*',
        }];

        assert_eq!(expected, actual)
    }
}

#[derive(Debug, PartialEq)]
struct Part {
    number: u32,
    component: char,
}

fn process_input(input: &str) -> Vec<Part> {
    vec![]
}

#[cfg(test)]
mod engine_tests {
    use super::*;

    #[test]
    fn collect_parts_single_line_no_parts_returns_empty_collection() {
        let input = "........";
        let actual = process_input(input);
        let expected: Vec<Part> = vec![];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_single_line_with_number_no_parts_returns_empty_collection() {
        let input = "123.....";
        let actual = process_input(input);
        let expected: Vec<Part> = vec![];

        assert_eq!(expected, actual)
    }

    #[test]
    fn collect_parts_single_line_with_number_and_symbol_returns_single_item() {
        let input = "123*.....";
        let actual = process_input(input);
        let expected = vec![Part {
            number: 123,
            component: '*',
        }];

        assert_eq!(expected, actual)
    }
}

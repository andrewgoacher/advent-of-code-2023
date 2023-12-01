pub fn get_number_from_string(input: &str) -> i32 {
    let chars = input.chars().filter(|c| c.is_numeric());

    let mut iter = chars.into_iter();
    let first = iter.next();
    let last = iter.next_back();
    get_first_and_last(first, last).unwrap_or(0)
}

fn get_first_and_last(first: Option<char>, last: Option<char>) -> Option<i32> {
    match first {
        Some(f) => match last {
            Some(l) => combine_chars(f, l),
            None => combine_chars(f, f),
        },
        None => None,
    }
}

fn combine_chars(first: char, last: char) -> Option<i32> {
    let collection = vec![first, last];
    collection.iter().collect::<String>().parse::<i32>().ok()
}

#[cfg(test)]
mod string_utils_tests {
    use super::*;

    macro_rules! get_number_from_string_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            let result = get_number_from_string(input);
            assert_eq!(expected, result)
        }
    )*
    }
}

    get_number_from_string_tests! {
        line_1: ("1abc2", 12),
        line_2: ("pqr3stu8vwx", 38),
        line_3: ("a1b2c3d4e5f", 15),
        line_4: ("treb7uchet", 77),
    }
}

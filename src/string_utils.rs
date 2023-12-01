pub fn get_number_from_string(input: &str) -> i32 {
    let chars = input.chars().filter(|c| c.is_numeric());

    let mut iter = chars.into_iter();
    let first = iter.next();
    let last = iter.next_back();
    if let Some(f) = first {
        if let Some(l) = last {
            let collection = vec![f, l];
            let number_as_string: String = collection.iter().collect();
            number_as_string.parse::<i32>().unwrap()
        } else {
            let collection = vec![f, f];
            let number_as_string: String = collection.iter().collect();
            number_as_string.parse::<i32>().unwrap()
        }
    } else {
        0
    }
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

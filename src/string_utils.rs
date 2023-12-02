use regex::Regex;

pub fn get_number_from_string(input: &str, capture_none_digits: bool) -> i32 {
    if capture_none_digits {
        get_text_or_number_from_string(input)
    } else {
        get_digit_only_from_string(input)
    }
}

fn get_digit_only_from_string(input: &str) -> i32 {
    let pattern = r"([1-9])";

    let re = Regex::new(pattern).unwrap();
    let captures: Vec<i32> = re
        .find_iter(input)
        .map(|c| c.as_str())
        .map(|c| match_str(c))
        .collect();

    let count = captures.len();

    let (tens, unit) = match count {
        0 => (0, 0),
        _ => (*captures.first().unwrap(), *captures.last().unwrap()),
    };

    (tens * 10) + unit
}

fn get_text_or_number_from_string(input: &str) -> i32 {
    // this is going to be filthy but regex lookbehind fell on its arse
    let mut captures = vec![];

    for i in 0..input.len() {
        if let Some(n) = str_contains_number(&input[i..]) {
            captures.push(n);
        }
    }

    let count = captures.len();

    let (tens, unit) = match count {
        0 => (0, 0),
        _ => (*captures.first().unwrap(), *captures.last().unwrap()),
    };

    (tens * 10) + unit
}

fn str_contains_number(str: &str) -> Option<i32> {
    match str {
        x if x.starts_with("one") => Some(1),
        x if x.starts_with("two") => Some(2),
        x if x.starts_with("three") => Some(3),
        x if x.starts_with("four") => Some(4),
        x if x.starts_with("five") => Some(5),
        x if x.starts_with("six") => Some(6),
        x if x.starts_with("seven") => Some(7),
        x if x.starts_with("eight") => Some(8),
        x if x.starts_with("nine") => Some(9),
        _ => str[0..1].parse::<i32>().ok(),
    }
}

fn match_str(str: &str) -> i32 {
    match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => str::parse::<i32>(str).unwrap_or(0),
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
            let (input,capture, expected) = $value;
            let result = get_number_from_string(input, capture);
            assert_eq!(expected, result)
        }
    )*
    }
}

    get_number_from_string_tests! {
        line_1_without_capture_nondigit: ("1abc2", false, 12),
        line_2_without_capture_nondigit: ("pqr3stu8vwx", false, 38),
        line_3_without_capture_nondigit: ("a1b2c3d4e5f", false,  15),
        line_4_without_capture_nondigit: ("treb7uchet", false, 77),
        line_5_without_capture_nondigit: ("two1nine", false, 11),
        line_6_without_capture_nondigit: ("eightwothree", false, 0),
        line_7_without_capture_nondigit: ("abcone2threexyz", false, 22),
        line_8_without_capture_nondigit: ("xtwone3four", false, 33),
        line_9_without_capture_nondigit: ("4nineeightseven2", false,  42),
        line_10_without_capture_nondigit: ("zoneight234", false, 24),
        line_11_without_capture_nondigit: ("7pqrstsixteen", false, 77),
    }

    get_number_from_string_tests! {
        line_1_with_capture_nondigit: ("1abc2", true, 12),
        line_2_with_capture_nondigit: ("pqr3stu8vwx", true, 38),
        line_3_with_capture_nondigit: ("a1b2c3d4e5f", true,  15),
        line_4_with_capture_nondigit: ("treb7uchet", true, 77),
        line_5_with_capture_nondigit: ("two1nine", true, 29),
        line_6_with_capture_nondigit: ("eightwothree", true, 83),
        line_7_with_capture_nondigit: ("abcone2threexyz", true, 13),
        line_8_with_capture_nondigit: ("xtwone3four", true, 24),
        line_9_with_capture_nondigit: ("4nineeightseven2", true,  42),
        line_10_with_capture_nondigit: ("zoneight234", true, 14),
        line_11_with_capture_nondigit: ("7pqrstsixteen", true, 76),
        line_12_with_capture_nondigit: ("eightoneight", true, 88),
        line_13_with_capture_nondigit: ("eighthree", true, 83),
    }
}

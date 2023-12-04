#[derive(Debug, PartialEq)]
pub struct Card {
    pub id: u32,
    pub winning_numbers: Vec<u32>,
    pub available_numbers: Vec<u32>,
}

fn split_rest(input: &str, separator: char) -> (&str, &str) {
    let mut parts = input.split(separator);
    let lhs = parts.next().expect("should have lhs");
    let rhs = parts.next().expect("should have rhs");

    (lhs.trim(), rhs.trim())
}

fn get_numbers(input: &str) -> Vec<u32> {
    let mut parts = input.split(' ');
    parts
        .into_iter()
        .map(|c| c.parse::<u32>())
        .map(|c| match c {
            Ok(t) => Some(t),
            Err(_) => None,
        })
        .filter_map(|c| c.map(|c| c))
        .collect()
}

pub fn win_pow(wins: Vec<u32>) -> u32 {
    let len = wins.len();

    if len == 0 {
        0
    } else {
        let pow: u32 = (len - 1) as u32;
        2u32.pow(pow)
    }
}

impl Card {
    pub fn from_string(input: &str) -> Self {
        let (id_str, numbers) = split_rest(input, ':');
        let id_str = id_str.replace("Card ", "");

        let id: u32 = id_str.parse().expect("Should have id");

        let (winning, all) = split_rest(numbers, '|');
        let mut winning = get_numbers(winning);
        let mut all = get_numbers(all);

        winning.sort();
        all.sort();

        Self {
            id,
            winning_numbers: winning,
            available_numbers: all,
        }
    }

    pub fn get_wins(&self) -> Vec<u32> {
        let mut wins = vec![];
        for winning_number in self.winning_numbers.iter() {
            if let Ok(x) = self.available_numbers.binary_search(&winning_number) {
                wins.push(winning_number.clone());
            }
        }

        wins
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;

    macro_rules! win_pow_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;

                    let result = win_pow(input);

                    assert_eq!(expected, result)
                }
            )*
        };
    }

    win_pow_tests! {
        pow_zero_returns_zero: (vec![], 0),
        pow_1_returns_1: (vec![1], 1),
        pow_2_returns_2: (vec![1,2], 2),
        pow_4_returns_8: (vec![1,2,3,4], 8),
    }

    macro_rules! split_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
            fn $name() {
                    let (input, split, lhs, rhs) = $value;
                    let (left, right) = split_rest(input, split);

                    assert_eq!(lhs, left);
                    assert_eq!(rhs, right)
                }
            )*
        }
    }

    split_tests! {
        split_with_pipe: ("this | or this", '|', "this", "or this"),
        split_with_colon: ("this or : this ", ':', "this or", "this"),
    }

    #[test]
    fn from_string_with_card_id_has_correct_id() {
        let input = "Card 1: 12 34 56 | 56 78";
        let card = Card::from_string(input);

        assert_eq!(1, card.id)
    }

    #[test]
    fn from_string_with_winning_numbers_has_correct_numbers() {
        let input = "Card 1: 12 34 56 | 56 78";
        let card = Card::from_string(input);

        let expected = vec![12, 34, 56];

        assert_eq!(expected, card.winning_numbers)
    }

    #[test]
    fn from_string_with_all_numbers_has_correct_numbers() {
        let input = "Card 1: 12 34 56 | 56 78";
        let card = Card::from_string(input);

        let expected = vec![56, 78];

        assert_eq!(expected, card.available_numbers)
    }

    #[test]
    fn from_string_with_no_winning_numbers_has_no_numbers() {
        let input = "Card 1: 12 34 56 | 57 78";
        let card = Card::from_string(input);

        let expected: Vec<u32> = vec![];
        let actual = card.get_wins();

        assert_eq!(expected, actual)
    }

    #[test]
    fn from_string_with_one_winning_numbers_has_one_number() {
        let input = "Card 1: 12 34 56 | 56 78";
        let card = Card::from_string(input);

        let expected: Vec<u32> = vec![56];
        let actual = card.get_wins();

        assert_eq!(expected, actual)
    }

    #[test]
    fn from_string_with_winning_numbers_has_numbers() {
        let input = "Card 1: 12 34 56 | 56 78 12";
        let card = Card::from_string(input);

        let expected: Vec<u32> = vec![12, 56];
        let actual = card.get_wins();

        assert_eq!(expected, actual)
    }
}

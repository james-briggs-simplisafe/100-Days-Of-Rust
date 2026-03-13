use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref DIGIT_TO_LETTERS: HashMap::<char, &'static str> = {
        HashMap::<char, &'static str>::from(
            [
                ('2', "abc"),
                ('3', "def"),
                ('4', "ghi"),
                ('5', "jkl"),
                ('6', "mno"),
                ('7', "pqrs"),
                ('8', "tuv"),
                ('9', "wxyz"),
            ]
        )
    };
}

pub struct PhoneNumberLetterCombos {
    string_options: Vec<&'static&'static str>,
    counters: Vec<usize>,
    exhausted: bool,
}

impl PhoneNumberLetterCombos {
    pub fn from_string(s: &str) -> Self {
        let string_options = s.chars().map(|ch| {
            DIGIT_TO_LETTERS.get(&ch).expect("Should be digits 2-9.")
        }).collect::<Vec<&&str>>();
        let counters = vec![0; string_options.len()];
        PhoneNumberLetterCombos {
            string_options,
            counters,
            exhausted: false,
        }
    }
}

impl Iterator for PhoneNumberLetterCombos {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None
        }

        let mut next_string = String::new();
    
        for i in 0..self.string_options.len() {
            next_string.push(
                self.string_options[i]
                .chars()
                .nth(self.counters[i])
                .expect("index defined as less than length")
            );
        };

        for i in (0..self.counters.len()).rev() {
            self.counters[i] += 1;
            if self.counters[i] >= self.string_options[i].len() {
                self.counters[i] = 0;
            } else {
                break;
            }
        }
        
        if self.counters.iter().all(|x|{*x == 0}) {
            self.exhausted = true;
        }

        if !next_string.is_empty() {
            Some(next_string)
        } else {
            None
        }
        
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23() {
        let actual: Vec<String> = PhoneNumberLetterCombos::from_string("23").collect();
        let expected: Vec<&str> = vec!["ad","ae","af","bd","be","bf","cd","ce","cf"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_empty() {
        let actual: Vec<String> = PhoneNumberLetterCombos::from_string("").collect();
        let expected: Vec<&str> = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let actual: Vec<String> = PhoneNumberLetterCombos::from_string("2").collect();
        let expected: Vec<&str> = vec!["a","b","c"];
        assert_eq!(actual, expected);
    }
}

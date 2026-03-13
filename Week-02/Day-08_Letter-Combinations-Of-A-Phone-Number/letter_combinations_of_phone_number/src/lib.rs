use core::panic;
use std::collections::HashMap;

pub fn letter_combinations_of_phone_number(number: &str) {
    let digit_maps = HashMap::<char, &'static str>::from(
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
    );

    let generating_strings = number.chars().map(|ch| {
        digit_maps.get(&ch).expect("Should be digits 2-9.")
    }).collect::<Vec<&&str>>();

    println!("{:?}", generating_strings);

    let mut counters = vec![0; generating_strings.len()];
    
    loop {
        let mut next_string = String::new();
        for i in 0..generating_strings.len() {
            next_string.push(
                generating_strings[i]
                .chars()
                .nth(counters[i])
                .expect("index defined as less than length")
            );
        }
        println!("{}", next_string);
        for i in (0..counters.len()).rev() {
            counters[i] += 1;
            if counters[i] >= generating_strings[i].len() {
                counters[i] = 0;
            } else {
                break;
            }
        }
        if (counters.iter().all(|x|{*x == 0})) {
            break;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        letter_combinations_of_phone_number("23");
        assert!(false);
    }
}

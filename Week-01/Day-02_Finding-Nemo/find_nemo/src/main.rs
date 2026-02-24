use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter a phrase to search or type 'exit' to quit");
        print!(">> ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line( &mut input)
            .expect("Failed to read from standard input.");

        if input.to_lowercase().trim() == "exit" {
            break;
        } else {
            println!("{}", find_nemo(&input));
        }
        println!("\n");
    }
    println!("Quitting application.")
    
}

fn find_nemo(input: &str) -> String {
        for (index, token) in input.split(char::is_whitespace).enumerate() {
            if token == "Nemo" {
                return format!("I found Nemo at {}!", index + 1);
            }
        } 
        return String::from("I can't find Nemo :(");
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_unique_nemo_at_five_positions() {
        assert_eq!(find_nemo("Nemo is only ever here"), "I found Nemo at 1!");
        assert_eq!(find_nemo("Only Nemo is ever here"), "I found Nemo at 2!");
        assert_eq!(find_nemo("Is only Nemo ever here"), "I found Nemo at 3!");
        assert_eq!(find_nemo("Is ever only Nemo here"), "I found Nemo at 4!");
        assert_eq!(find_nemo("Here is only ever Nemo"), "I found Nemo at 5!");
    }

    #[test]
    fn returns_first_nemo() {
        assert_eq!(find_nemo("Nemo Nemo Nemo"), "I found Nemo at 1!")
    }

    #[test]
    fn nemo_is_case_sensitive() {
        assert_eq!(find_nemo("Nemo"), "I found Nemo at 1!");
        assert_eq!(find_nemo("nemo"), "I can't find Nemo :(");
        assert_eq!(find_nemo("nEmo"), "I can't find Nemo :(");
        assert_eq!(find_nemo("nEMO"), "I can't find Nemo :(");
        assert_eq!(find_nemo("nEmO"), "I can't find Nemo :(");
        assert_eq!(find_nemo("NEMO"), "I can't find Nemo :(");
    }

    #[test]
    fn nemo_must_be_full_token() {
        assert_eq!(find_nemo("Nemo"), "I found Nemo at 1!");
        assert_eq!(find_nemo("Nemo!"), "I can't find Nemo :(");
        assert_eq!(find_nemo("NemoD sdrawkcab"), "I can't find Nemo :(");
    }
}

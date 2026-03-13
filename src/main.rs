use rand::prelude::*;
use std::fmt;
use std::io;

enum InputErrors {
    Zero,
    MinPassLenRequired,
}

impl fmt::Display for InputErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputErrors::Zero => write!(f, "Password length cannot be zero"),
            InputErrors::MinPassLenRequired => {
                write!(f, "Minimum password length is 5, try better")
            }
        }
    }
}

const CHARS_TO_USE: [char; 77] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '@',
    '#', '$', '%', '^', '&', '*', '<', '>', '?', '[', ']', '{', '}', 'A', 'B', 'C', 'D', 'E', 'F',
    'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
    'Z',
];

fn input_pass_len() -> Result<u8, InputErrors> {
    println!("Enter the password length");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    match input.trim().parse::<u8>() {
        Ok(number) if number == 0 => Err(InputErrors::Zero),

        Ok(number) if number < 5 => Err(InputErrors::MinPassLenRequired),

        Ok(number) => {
            println!("Ok");

            Ok(number)
        }

        Err(err) => {
            eprintln!("Input Error {}", err);
            std::process::exit(1);
        }
    }
}

fn pass_gen(req_pass_len: u8) -> String {
    let mut pass: String = String::new();
    let mut rng: ThreadRng = rand::rng();
    let char_arr_len: usize = CHARS_TO_USE.len();

    for _ in 0..req_pass_len {
        let chars_idx = rng.random_range(0..char_arr_len);
        pass.push(CHARS_TO_USE[chars_idx]);

        if pass.len() == req_pass_len as usize {
            break;
        }
    }
    pass
}

fn main() {
    println!("THIS is a password  generator");
    let pass_len = match input_pass_len() {
        Ok(p_len) => p_len,
        Err(err) => {
            eprintln!("Input Error: {}", err);
            std::process::exit(1);:
        }
    };
    println!("{}", pass_len);
    let pass: String = pass_gen(pass_len);

    println!("Your password is {}", pass);

    let out = std::process::Command::new("cr")
        .arg("")


}

use anyhow::{self, Result};
use rand::seq::{IteratorRandom, SliceRandom};
use zxcvbn::zxcvbn;

const UPPER: &str = "ABCDEFGHIJKMNPQRSTUVWXYZ";
const LOWER: &str = "abcdefghijkmnpqrstuvwxyz";
const NUMBERS: &str = "123456789";
const SYMBOLS: &str = "!@#$%^&*-_=+?/";

pub fn process_gen_pass(len: u8, upper: bool, lower: bool, num: bool, sym: bool) -> Result<String> {
    let len = len.clamp(4, 255); // constrain between 4 and 255
    let mut charset = String::new();
    let mut at_least_one = String::new();
    let scopes = [
        (UPPER, upper),
        (LOWER, lower),
        (NUMBERS, num),
        (SYMBOLS, sym),
    ];
    for (set, use_set) in scopes.iter() {
        if *use_set {
            charset.push_str(set);
            at_least_one.push(random_from_set(set));
        }
    }

    if charset.is_empty() {
        return Err(anyhow::anyhow!("No character set selected"));
    }
    if len as usize <= at_least_one.len() {
        return Err(anyhow::anyhow!(
            "Password length too short for selected character types"
        ));
    }

    let mut rng = rand::rng();
    let mut password = at_least_one;

    for _ in 0..(len as usize - password.len()) {
        password.push(charset.chars().choose(&mut rng).unwrap());
    }
    let password = shuffled_string(password);
    // print password strength
    let estimate = zxcvbn(&password, &[]);
    // eprint!("Password strength: {:?}\n", estimate.score());
    Ok(format!(
        "password: {}\nstrength: {} \n(reference range: 0 (too guessable) - 4 (very unguessable))",
        password,
        estimate.score()
    ))
}

fn random_from_set(charset: &str) -> char {
    let mut rng = rand::rng();
    charset.chars().choose(&mut rng).unwrap()
}

fn shuffled_string(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut rng = rand::rng();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}

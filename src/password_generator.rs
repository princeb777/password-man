use colored::Colorize;
use rand::{
    Rng,
    seq::{IndexedRandom, SliceRandom},
};

const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%&*";

pub fn new_password(password_length: usize) -> String {
    let mut rng = rand::rng();
    let mut password: Vec<char> = Vec::new();

    if password_length < 4 {
        panic!("Password length must be at least 4 to contain all character types.");
    }

    password.push(LOWER.chars().nth(rng.random_range(0..LOWER.len())).unwrap());
    password.push(UPPER.chars().nth(rng.random_range(0..UPPER.len())).unwrap());
    password.push(
        NUMBERS
            .chars()
            .nth(rng.random_range(0..NUMBERS.len()))
            .unwrap(),
    );
    password.push(
        SYMBOLS
            .chars()
            .nth(rng.random_range(0..SYMBOLS.len()))
            .unwrap(),
    );

    let mut all_chars = String::new();
    all_chars.push_str(LOWER);
    all_chars.push_str(UPPER);
    all_chars.push_str(NUMBERS);
    all_chars.push_str(SYMBOLS);
    let all_chars: Vec<char> = all_chars.chars().collect();

    for _ in 0..password_length - 4 {
        password.push(*all_chars.choose(&mut rng).unwrap());
    }

    password.shuffle(&mut rng);

    password.into_iter().collect()
}

pub struct PasswordManager;

impl PasswordManager {
    pub fn show_from_entries(entries: Vec<String>) {
        if entries.is_empty() {
            println!("{}", "⚠️  No passwords stored yet.".yellow().bold());
            return;
        }

        println!("\n{}", "🔑 Stored Passwords".bright_yellow().bold());
        println!(
            "{}",
            "=====================================================".bright_black()
        );
        println!(
            "{} | {} | {}",
            "Name".cyan().bold(),
            "Password".blue().bold(),
            "Description".green().bold()
        );
        println!(
            "{}",
            "-----------------------------------------------------".bright_black()
        );

        for line in entries {
            let parts: Vec<&str> = line.split("||").map(|s| s.trim()).collect();
            if parts.len() == 3 {
                println!(
                    "{} | {} | {}",
                    parts[0].cyan(),
                    parts[1].blue(),
                    if parts[2].is_empty() {
                        "-".bright_black().to_string()
                    } else {
                        parts[2].green().to_string()
                    }
                );
            }
        }

        println!(
            "{}",
            "=====================================================\n".bright_black()
        );
    }
}

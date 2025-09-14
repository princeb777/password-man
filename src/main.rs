use colored::Colorize;
use std::io::{self, Write};

mod password_generator;
mod vault;

use password_generator::PasswordManager;
use vault::PasswordVault;

fn main() {
    // Ask for master password at startup
    println!("{}", "🔑 Enter master password:".bright_yellow().bold());
    print!("➜ ");
    io::stdout().flush().unwrap();

    let mut master_pwd = String::new();
    io::stdin().read_line(&mut master_pwd).unwrap();
    let master_pwd = master_pwd.trim().to_string();

    // Load vault
    let mut vault = PasswordVault::load(master_pwd, "passwords.txt");

    println!("\n{}", "🔐 Password Manager CLI".bright_yellow().bold());
    println!("{}", "[1] Generate a new password".cyan());
    println!("{}", "[2] Show all saved passwords".cyan());
    println!("{}", "[3] Exit".cyan());
    print!("{} ", "➜ Choose an option:".bright_yellow());
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            let password = password_generator::new_password(12);
            println!(
                "\n{}: {}",
                "Generated password".green().bold(),
                password.blue().bold()
            );

            println!("\n{}", "Do you want to save this password? (y/n)".cyan());
            print!("➜ ");
            io::stdout().flush().unwrap();

            let mut save_choice = String::new();
            io::stdin().read_line(&mut save_choice).unwrap();

            if save_choice.trim().eq_ignore_ascii_case("y") {
                println!("Enter a name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                println!("Enter a description (optional): ");
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();

                let entry = format!("{} || {} || {}", name.trim(), password, desc.trim());

                vault.add_entry(&entry);
                vault.save();
                println!("{}", "✅ Password saved.".green().bold());
            } else {
                println!("{}", "❌ Password not saved.".red());
            }
        }
        "2" => {
            let entries = vault.get_entries();
            PasswordManager::show_from_entries(entries);
        }
        "3" => {
            println!("{}", "👋 Goodbye!".bright_green());
        }
        _ => println!("{}", "❌ Invalid choice".red()),
    }
}

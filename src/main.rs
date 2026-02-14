use std::io::{self, Write};
mod account_utils;
use crate::account_utils as acut;
use rpassword::read_password;

fn main() {
    println!("Hello user this is test bank! Please login into the account!");
    print!("Do u have account? (y/n): ");
    io::stdout().flush().unwrap();

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    let answer = answer.trim();

    if answer == "n" {
        let mut accounts = acut::load_account("accounts.json").unwrap_or_else(|_| Vec::new());
        let next_id = accounts.iter().map(|a| a.account_id).max().unwrap_or(0) + 1;
        let new_account = acut::creating_user(next_id);
        println!("Ur account succesfully created!");
        accounts.push(new_account);

        match acut::save_account(&accounts, "accounts.json") {
            Ok(_) => println!("Saved to accounts.json"),
            Err(e) => eprintln!("Save error: {e}"),
        }
    } else if answer == "y" {
        let accounts = acut::load_account("accounts.json").expect("Smth wrong!");

        print!("Please write ur name: ");
        io::stdout().flush().unwrap();

        let mut account_name = String::new();
        io::stdin().read_line(&mut account_name).unwrap();
        let account_name = account_name.trim();

        let found_account = accounts.iter().find(|acc| acc.name == account_name);

        match found_account {
            Some(account) => {
                print!("Account {} found. Please Write ur password: ", account.name);
                io::stdout().flush().unwrap();
                let account_password = read_password().unwrap();
                let account_password = account_password.trim().to_string();
                if account_password == account.password {
                    io::stdout().flush().unwrap();
                    println!("Welcome back, {}!", account.name);
                    let mut user = account.clone();
                    acut::panel(&mut user);
                } else {
                    println!("Wrong password!");
                }
                
            },  
            None => println!("Account not found!"),
        }
        ;
    } else {
        println!("Please enter only 'y' or 'n'");
    }
}

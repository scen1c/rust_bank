use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use std::fs;

mod account_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BankAccountRust {
    name: String,
    account_id: u32,
    balance: i32,
    email: String,
    phone: u32,
    password: String,
}


fn main() {
    println!("Hello user this is test bank! Please login into the account!");
    print!("Do u have account? (y/n): ");
    io::stdout().flush().unwrap();

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    let answer = answer.trim();

    if answer == "n" {
        let new_account = account_utils::creating_user();
        println!("Ur account succesfully created!");

        let mut accounts = account_utils::load_account("accounts.json").unwrap_or_else(|_| Vec::new());
        accounts.push(new_account);

        match account_utils::save_account(&accounts, "accounts.json") {
            Ok(_) => println!("Saved to accounts.json"),
            Err(e) => eprintln!("Save error: {e}"),
        }
    } else if answer == "y" {
        let accounts = account_utils::load_account("accounts.json").unwrap_or_else(|_| Vec::new());

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
                let mut account_password = String::new();
                io::stdin().read_line(&mut account_password).unwrap();
                let account_password = account_password.trim().to_string();
                if account_password == account.password {
                    io::stdout().flush().unwrap();
                    println!("Welcome back, {}!", account.name);
                    account_utils::information(&account);
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

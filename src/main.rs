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
        let new_account = creating_user();
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
            Some(account) => println!("Found: {:?}, balance={}", account.name, account.balance),
            None => println!("Account not found!"),
        }
    } else {
        println!("Please enter only 'y' or 'n'");
    }
}
fn creating_user() -> BankAccountRust {
    let mut name = String::new();
    let mut email = String::new();
    let mut phone_string = String::new();
    let mut password = String::new();
    print!("Lets create ur account. Whats ur name?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Error");
    let name = name.trim().to_string();
    print!("Whats ur email?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).expect("Error");
    let email = email.trim().to_string();
    print!("Whats ur phone number?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut phone_string).expect("Error");
    let phone = phone_string.trim().parse().expect("Error");
    print!("Lets write ur password! Create password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).expect("Error");
    let password = password.trim().to_string();
    BankAccountRust { 
        name, 
        account_id: 1, 
        balance: 0, 
        email, 
        phone, 
        password,
    }
}


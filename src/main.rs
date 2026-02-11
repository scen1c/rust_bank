use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use std::fs;
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
    let mut json_string: Option<String> = None;

    println!("Hello user this is test bank! Please login into the account!");
    print!("Do u have account? (y/n): ");
    io::stdout().flush().unwrap();
    let mut answer = String::new();
    io::stdin()
    .read_line(&mut answer)
    .expect("Please enter y or n");

    if answer.trim() == "n" {
        let new_account = creating_user();
        println!("Ur account succesfully created!");
        let json_string = serde_json::to_string_pretty(&new_account);
        match serde_json::to_string_pretty(&new_account) {
            Ok(json) => println!("{json}"),
            Err(e) => eprintln!("JSON error: {e}"),
        }
    };
    println!("{:#?}", json_string)
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


use std::fs;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BankAccountRust {
    pub name: String,
    pub account_id: u32,
    pub balance: i128,
    pub is_admin: bool,
    pub email: String,
    pub phone: u32,
    pub password: String,
}

pub fn save_account(accounts: &Vec<BankAccountRust>, path: &str) -> io::Result<()> {
    let json = serde_json::to_string_pretty(accounts).unwrap();
    fs::write(path, json)?;
    Ok(())
}

pub fn load_account(path: &str) -> io::Result<Vec<BankAccountRust>> {
    let file_content = fs::read_to_string(path)?;
    let accounts: Vec<BankAccountRust> = serde_json::from_str(&file_content).unwrap();
    Ok(accounts)
}

pub fn creating_user(account_id: u32) -> BankAccountRust {
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
        account_id, 
        balance: 0,
        is_admin: false, 
        email, 
        phone, 
        password,
    }
}

pub fn information(user: &BankAccountRust) -> () {
    let mut option = String::new();
    println!("Which type of info u need to know?
    1.Name of Account
    2.Account ID
    3.Balance
    4.Email
    5.Phone Number
    6.Password"
    );
    print!("Choose from 1 to 6: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut option).unwrap();
    let option = option.trim().parse().expect("Error it has to be numeral from 1 to 6");
    match option {
        1 => println!("Name account is: {}", user.name),
        2 => println!("Account ID is: {}", user.account_id),
        3 => if user.is_admin {
                println!("Balance of account is: Unlimited");
            } else {
            println!("Balance of account is: {}", user.balance);
            },
        4 => println!("Email of account is: {}", user.email),
        5 => println!("Phone number is: {}", user.phone),
        6 => println!("Password of account is: {}", user.password),
        _ => println!("You have chosen wrong number, try again!"),
    }

}

pub fn balance_of_admin(admin: &mut BankAccountRust) {
    if admin.is_admin || admin.balance == i128::MIN {
        admin.balance = i128::MAX;
        }
    }

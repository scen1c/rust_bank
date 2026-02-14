use std::fs;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BankAccountRust {
    pub name: String,
    pub account_id: u32,
    pub balance: HashMap<String, f64>,
    pub is_admin: bool,
    pub email: String,
    pub phone: u32,
    pub password: String,
}

impl BankAccountRust {
    pub fn new() -> Self {
        let mut balance = HashMap::new();
        balance.insert("USD".to_string(), 0.0);
        balance.insert("EUR".to_string(), 0.0);
        

        Self {
            name: String::new(),
            account_id: 0,
            balance,
            is_admin: false,
            email: String::new(),
            phone: 0,
            password: String::new(),
        }
    }
}

pub fn save_account(accounts: &Vec<BankAccountRust>, path: &str) -> io::Result<()> {
    let json = serde_json::to_string_pretty(accounts).unwrap();
    fs::write(path, json)?;
    Ok(())
}

pub fn load_account(path: &str) -> io::Result<Vec<BankAccountRust>> {
    let file_content = fs::read_to_string(path)?;
    let accounts: Vec<BankAccountRust> = serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new());
    Ok(accounts)
}

pub fn creating_user(account_id: u32) -> BankAccountRust {
    let mut user = BankAccountRust::new();  
    user.account_id = account_id;

    print!("Lets create ur account. Whats ur name?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user.name).unwrap();
    user.name = user.name.trim().to_string();

    print!("Whats ur email?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user.email).unwrap();
    user.email = user.email.trim().to_string();

    let mut phone_string = String::new();
    print!("Whats ur phone number?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut phone_string).unwrap();
    user.phone = phone_string.trim().parse().unwrap();

    print!("Create password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user.password).unwrap();
    user.password = user.password.trim().to_string();

    user
}

pub fn account_info(user: &mut BankAccountRust) {
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
    let option: i8 = option.trim().parse().expect("Error it has to be numeral from 1 to 6");
    match option {
        1 => println!("Name account is: {}", user.name),
        2 => println!("Account ID is: {:06}", user.account_id),
        3 => println!("Balance of account is: {:?}", user.balance),
        4 => println!("Email of account is: {}", user.email),
        5 => println!("Phone number is: {}", user.phone),
        6 => println!("Password of account is: {}", user.password),
        _ => println!("You have chosen wrong number, try again!"),
    }

}

pub fn admin_account_info(admin: &mut BankAccountRust) {
    let mut option1 = String::new();
    println!("Which type of info u need to know?
    1.Name of Account
    2.Account ID
    3.Balance
    4.Email
    5.Phone Number
    6.Password
    7.Accounts.json info"
    );
    print!("Choose from 1 to 6: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut option1).unwrap();
    let option1: i8 = option1.trim().parse().expect("Error it has to be numeral from 1 to 6");
    match option1 {
        1 => println!("Name account is: {}", admin.name),
        2 => println!("Account ID is: {:06}", admin.account_id),
        3 => {
            println!("Balance of account is: {:?}", admin.balance)
        },
        4 => println!("Email of account is: {}", admin.email),
        5 => println!("Phone number is: {}", admin.phone),
        6 => println!("Password of account is: {}", admin.password),
        7 => {
            let mut option2 = String::new();
            println!("What do u want?
            1. Search account"
            );
            print!("Choose from 1 to 1: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut option2).unwrap();
            let option2: i8 = option2.trim().parse().expect("Error it has to be numeral from 1 to 1");
            match option2 {
                1 => {
                    let accounts = load_account("accounts.json").expect("Smth wrong!");
                    
                    let mut name_finder = String::new();
                    print!("What name u want find?: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut name_finder).unwrap();
                    let found_account = accounts.iter().find(|acc| acc.name == name_finder.trim());
                    match found_account {
                        Some(account) =>  {
                            println!("Yes, we found the name!");
                            println!("{:#?}", account)
                        },
                        None => println!("There is no account with this name"),
                }
            },
            _ => println!("You have chosen wrong number, try again!")
        }
        
    }
        _ => println!("You have chosen wrong number, try again!"),

            }

}


pub fn top_up(user: &mut BankAccountRust) {
    let mut accounts = load_account("accounts.json").unwrap();

    let mut currency = String::new();
    print!("Enter currency (USD/EUR/etc): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut currency).unwrap();
    let currency = currency.trim().to_uppercase();
    let mut money = String::new();
    print!("How much money do u want to deposit?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut money).unwrap();
    let money: f64 = money.trim().parse().expect("Please write number");

    // если валюты нет — создаём
    let balance_entry = user.balance.entry(currency.clone()).or_insert(0.0);
    *balance_entry += money;

    println!("Deposited {} {}", money, currency);
    println!("New balance: {}", balance_entry);

    let acc = accounts.iter_mut().find(|a| a.name == user.name).unwrap();
    acc.balance = user.balance.clone();
    save_account(&accounts, "accounts.json").unwrap();
}

pub fn withdraw(user: &mut BankAccountRust) {
    let mut currency = String::new();
    print!("Enter currency (USD/EUR/etc): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut currency).unwrap();
    let currency = currency.trim().to_uppercase();

    let mut money = String::new();
    print!("How much money do u want to withdraw?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut money).unwrap();
    let money: f64 = money.trim().parse().expect("Please write number");

    match user.balance.get_mut(&currency) {
        Some(balance) => {
            if *balance >= money {
                *balance -= money;
                println!("Withdrawn {} {}", money, currency);
                println!("Remaining balance: {}", balance);
            } else {
                println!("Not enough funds!");
            }
        }
        None => println!("You do not have this currency account."),
    }

    let mut accounts = load_account("accounts.json").unwrap();
    let acc = accounts.iter_mut().find(|a| a.name == user.name).unwrap();
    acc.balance = user.balance.clone();
    save_account(&accounts, "accounts.json").unwrap();
}

pub fn panel(user: &mut BankAccountRust) {
    let mut option = String::new();
    println!("This is panel of RustBank(Pet project)
    This is options that u can do on ur account:
    1.Check information
    2.Top up balance
    3.Withdraw money"
    );
    print!("Choose option from 1 to 3: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut option).unwrap();
    let option = option.trim().parse().expect("Error it has to be numeral from 1 to 6");
    match option {
        1 => {
            if user.is_admin {
                        let mut admin = user.clone(); 
                        admin_account_info(&mut admin);
                    }
                    else {
                        let mut user = user.clone();
                        account_info(&mut user);
                    }
        },
        2 => top_up(user),
        3 => withdraw(user),

        _ => println!("Error, try again")
    };
}

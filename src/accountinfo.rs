use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::saveload as sl;
use crate::money_manipulations as mm;
use crate::password as pw;

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
    user.password = pw::hash_password(&user.password);

    user
}

pub fn panel(user: &mut BankAccountRust) {
    let mut option = String::new();
    println!("This is panel of RustBank(Pet project)
    This is options that u can do on ur account:
    1.Check information
    2.Top up balance
    3.Withdraw mone
    4.Convertation
    5.Transfer money to"
    );
    print!("Choose option from 1 to 5: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut option).unwrap();
    let option = option.trim().parse().expect("Error it has to be numeral from 1 to 5");
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
        2 => mm::top_up(user),
        3 => mm::withdraw(user),
        4 => mm::convertation(user),
        5 => mm::transfer_to_account(user),
        _ => println!("Error, try again")
    };
}

pub fn account_info(user: &mut BankAccountRust) {
    let mut option = String::new();
    println!("Which type of info u need to know?
    1.Name of Account
    2.Account ID
    3.Balance
    4.Email
    5.Phone Number
    6.Password
    7.Change password"
    );
    print!("Choose from 1 to 7: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut option).unwrap();
    let option: i8 = option.trim().parse().expect("Error it has to be numeral from 1 to 7");
    match option {
        1 => println!("Name account is: {}", user.name),
        2 => println!("Account ID is: {:06}", user.account_id),
        3 => println!("Balance of account is: {:#?}", user.balance),
        4 => println!("Email of account is: {}", user.email),
        5 => println!("Phone number is: {}", user.phone),
        6 => println!("Password of account is: {}", user.password),
        7 => pw::change_password(user),
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
            println!("Balance of account is: {:#?}", admin.balance)
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
                    let accounts = sl::load_account("accounts.json").expect("Smth wrong!");
                    
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

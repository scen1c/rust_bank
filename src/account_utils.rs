use std::fs;
use std::io::{self, Write};
use crate::BankAccountRust;

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

pub fn creating_user() -> BankAccountRust {
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


pub fn information(user: &BankAccountRust) -> () {
    let mut option = String::new();
    println!("Which type of info u need to know?\n
    1.Name of Account\n
    2.Account ID\n
    3.Balance\n
    4.Email\n
    5.Phone Number\n
    6.Password\n
    
    ");
    print!("Choose from 1 to 6: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut option).unwrap();
    let option = option.trim().parse().expect("Error it has to be numeral from 1 to 6");
    match option {
        1 => println!("Name account is: {}", user.name),
        2 => println!("Account ID is: {}", user.account_id),
        3 => println!("Balance of account is: {}", user.balance),
        4 => println!("Email of account is: {}", user.email),
        5 => println!("Phone number is: {}", user.phone),
        6 => println!("Password of account is: {}", user.password),

        _ => println!("You have chosen wrong number, try again!"),
    }

}

use crate::accountinfo::{self as ai, BankAccountRust};
use std::io::{self, Write};
use crate::saveload as sl;


pub fn top_up(user: &mut ai::BankAccountRust) {
    let mut accounts = sl::load_account("accounts.json").unwrap();

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

    let balance_entry = user.balance.entry(currency.clone()).or_insert(0.0);
    *balance_entry += money;

    println!("Deposited {} {}", money, currency);
    println!("New balance: {}", balance_entry);

    let acc = accounts.iter_mut().find(|a| a.name == user.name).unwrap();
    acc.balance = user.balance.clone();
    sl::save_account(&accounts, "accounts.json").unwrap();
}

pub fn top_up_test(user: &mut ai::BankAccountRust, currency: &str, money: f64) -> Result<(), String> {
    let currency = currency.trim().to_uppercase();

    if money <= 0.0 {
        return Err("Money must be > 0".to_string());
    }

    let balance_entry = user.balance.entry(currency).or_insert(0.0);
    *balance_entry += money;

    Ok(())
}

pub fn withdraw(user: &mut ai::BankAccountRust) {
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

    let mut accounts = sl::load_account("accounts.json").unwrap();
    let acc = accounts.iter_mut().find(|a| a.name == user.name).unwrap();
    acc.balance = user.balance.clone();
    sl::save_account(&accounts, "accounts.json").unwrap();
}

pub fn convertation(user: &mut ai::BankAccountRust) {
    
    const EUR_TO_USD: f64 = 1.18;
    const USD_TO_EUR: f64 = 1.0 / 1.18;

    let mut from = String::new();
    let mut to = String::new();

    print!("Convert FROM (USD/EUR): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut from).unwrap();
    let from = from.trim().to_uppercase();

    print!("Convert TO (USD/EUR): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut to).unwrap();
    let to = to.trim().to_uppercase();

    if from == to {
        println!("You selected same currency.");
        return;
    }

    let rate = match (from.as_str(), to.as_str()) {
        ("EUR", "USD") => EUR_TO_USD,
        ("USD", "EUR") => USD_TO_EUR,
        _ => {
            println!("Unsupported currency pair.");
            return;
        }
    };

    let balance_from = match user.balance.get_mut(&from) {
        Some(b) => b,
        None => {
            println!("You don't have this currency.");
            return;
        }
    };

    let mut money = String::new();
    print!("How much money you want to convert: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut money).unwrap();
    let money: f64 = match money.trim().parse() {
        Ok(m) => m,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    if *balance_from < money {
        println!("Not enough balance.");
        return;
    }

    *balance_from -= money;

    let converted = money * rate;

    user.balance.entry(to.clone()).and_modify(|b| *b += converted).or_insert(converted);

    let mut accounts = sl::load_account("accounts.json").unwrap();
    let acc = accounts.iter_mut().find(|a| a.name == user.name).unwrap();
    acc.balance = user.balance.clone();
    sl::save_account(&accounts, "accounts.json").unwrap();
    println!("Successfully converted!");
}


pub fn transfer_to_account(user: &mut ai::BankAccountRust) {
    let mut accounts = sl::load_account("accounts.json").unwrap();
    let mut whom = String::new();
    print!("Who do u wna send money to?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut whom).unwrap();
    let whom = whom.trim().to_string();
    let person = accounts.iter_mut().find(|a| a.name == whom);
    if person.is_none() {
        println!("There is no account with this name");
        return;
    }
    let person = person.unwrap();
    let mut value = String::new();
    print!("From which balance do u wna send(EUR/USD): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut value).unwrap();
    let currency = value.trim().to_string().to_uppercase();
    if currency != "USD" && currency != "EUR" {
        println!("Error format");
        return;
    };
    let mut money = String::new();
    print!("How much money do u wna send?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut money).unwrap();
    let money: f64= money.trim().parse().expect("Error");
    let sender_balance = user.balance.entry(currency.clone()).or_insert(0.0);
    if *sender_balance < money {
        println!("Not enough money from ur {currency} balance: {sender_balance}");
        return;
    };
    *sender_balance -= money;

    
    person.balance.entry(currency.clone()).and_modify(|b| *b += money).or_insert(money);

    let mut accounts = sl::load_account("accounts.json").unwrap();
    let acc1 = accounts.iter_mut().find(|a| a.name == user.name).unwrap();
    acc1.balance = user.balance.clone();
    let acc2 = accounts.iter_mut().find(|b| b.name == person.name).unwrap();
    acc2.balance = person.balance.clone();
   sl:: save_account(&accounts, "accounts.json").unwrap();
    println!("Successfully send it to {0:?}!", person.name);

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_user() -> ai::BankAccountRust {
        let mut balance = HashMap::new();
        balance.insert("USD".to_string(), 500.0);
        balance.insert("EUR".to_string(), 300.0);
        ai::BankAccountRust {
            name: "Michael".to_string(),
            account_id: 10,
            balance: balance,
            password: "123456789".to_string(),
            is_admin: false,
            email: "michael@mail.com".to_string(),
            phone: 0000
        }
    }

    #[test]
    fn top_up_adds_money_usd() {
        let mut user = make_user();
        top_up_test(&mut user, "USD", 20.0).unwrap();
        assert_eq!(*user.balance.get("USD").unwrap(), 520.0);

    }
}
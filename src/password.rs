use std::io::{self, Write};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::accountinfo as acut;
use crate::saveload as sl;
use rpassword::read_password;
pub fn hash_password(plain: &str) -> String {
    hash(plain, DEFAULT_COST).expect("Failed to hash password")
}

pub fn verify_password(plain: &str, hashed: &str) -> bool {
    verify(plain, hashed).unwrap_or(false)
}

pub fn is_bcrypt_hash(password: &str) -> bool {
    password.starts_with("$2a$")
        || password.starts_with("$2b$")
        || password.starts_with("$2y$")
}

pub fn migrate_passwords(accounts: &mut Vec<acut::BankAccountRust>) -> bool {
    let mut changed = false;

    for acc in accounts.iter_mut() {
        if !is_bcrypt_hash(&acc.password) {
            acc.password = hash_password(&acc.password);
            
            changed = true;
        }
    }
    if changed {
        sl::save_account(accounts, "accounts.json").unwrap();
    }
    changed
}

pub fn change_password(account: &mut acut::BankAccountRust) {
    print!("Write ur password!: ");
    io::stdout().flush().unwrap();
    let prev_password = read_password().unwrap();
    let prev_password = prev_password.trim();
    if verify_password(&prev_password, &account.password) {
        let mut new_password = String::new();
        print!("Write ur new password: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut new_password).unwrap();
        let new_password = new_password.trim();
        account.password = hash_password(&new_password);
        let mut accounts = sl::load_account("accounts.json").unwrap();
        let acc = accounts.iter_mut().find(|a| a.name == account.name).unwrap();
        acc.password = account.password.clone();
        sl::save_account(&accounts, "accounts.json").unwrap();
    } else {
        println!("This is not ur password")
    };

}
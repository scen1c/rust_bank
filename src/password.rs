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


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_user() -> acut::BankAccountRust {
        let mut balance = HashMap::new();
        balance.insert("USD".to_string(), 500.0);
        balance.insert("EUR".to_string(), 300.0);
        acut::BankAccountRust {
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
    fn hash_password_check() {
        let hash_check = hash_password("1234567890");
        let plain = "1234567890";
        assert!(is_bcrypt_hash(&hash_check));
        assert_ne!(hash_check, plain);                
        assert!(verify_password(plain, &hash_check));  
        assert!(!verify_password("wrong", &hash_check));
    }

}
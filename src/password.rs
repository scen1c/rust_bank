use bcrypt::{hash, verify, DEFAULT_COST};
use crate::accountinfo as acut;
use crate::saveload as sl;

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
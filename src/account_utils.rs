use std::fs;
use std::io;

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
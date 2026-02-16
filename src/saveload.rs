use crate::accountinfo as acin;
use std::io;
use std::fs;




pub fn save_account(accounts: &Vec<acin::BankAccountRust>, path: &str) -> io::Result<()> {
    let json = serde_json::to_string_pretty(accounts).unwrap();
    fs::write(path, json)?;
    Ok(())
}

pub fn load_account(path: &str) -> io::Result<Vec<acin::BankAccountRust>> {
    let file_content = fs::read_to_string(path)?;
    let accounts: Vec<acin::BankAccountRust> = serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new());
    Ok(accounts)
}
use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
const DB_PATH: &str = "./data/db.json";


#[derive(Serialize, Deserialize, Clone)]
struct UserAccount {
    id: usize,
    name: String,
    active: bool,
    date_of_birth: String,
    address: String,
    social_security: u32,
    created_at: DateTime<Utc>,
    last_accessed: DateTime<Utc>,
}
struct CheckingAccount {
    account_owner_id: usize,
    account_type: String,
    balance: usize,
    created_at: DateTime<Utc>,
    last_transaction: DateTime<Utc>,
}


#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

fn main() -> Result<(), Error> {
    let mut user_accounts: Vec<UserAccount> = Vec::new();
    let mut checking_accounts: Vec<CheckingAccount> = Vec::new();

    let user = create_user_account(
        "John".to_string(),
        "01/01/1990".to_string(),
        "123 Main St".to_string(),
        123456789,
    );
    user_accounts.push(user);

    let checking_account = create_checking_account(user_accounts[0].id);
    checking_accounts.push(checking_account);

    save_to_db(&user_accounts, &checking_accounts)?;
    println!("User and checking account created successfully.");
    create_user_account("John".to_string(), "01/01/1990".to_string(), "123 Main St".to_string(), 123456789);
}

fn save_to_db(user_accounts: &Vec<UserAccount>, checking_accounts: &Vec<CheckingAccount>) -> Result<(), Error> {
    let db_data = serde_json::json!({
        "user_accounts": user_accounts,
        "checking_accounts": checking_accounts,
    });

    let db_json = serde_json::to_string_pretty(&db_data)?;
    fs::write(DB_PATH, db_json)?;
    Ok(())
}

fn create_user_account(
    name: String,
    date_of_birth: String,
    address: String,
    social_security: u32,
) -> UserAccount {
    UserAccount {
        id: 1,
        name: name,
        active: true,
        date_of_birth: date_of_birth,
        address: address,
        social_security: social_security,
        created_at: chrono::offset::Utc::now(),
        last_accessed: chrono::offset::Utc::now(),
    }
}

fn create_checking_account(account_owner_id: usize) -> CheckingAccount {
    CheckingAccount {
        account_owner_id,
        account_type: "Checking".to_string(),
        balance: 0,
        created_at: chrono::offset::Utc::now(),
        last_transaction: chrono::offset::Utc::now(),
    }
}

fn get_account_details(user_accounts: &Vec<UserAccount>, user_id: usize) {
    if let Some(user) = user_accounts.iter().find(|&u| u.id == user_id) {
        println!("User ID: {}", user.id);
        println!("Name: {}", user.name);
        println!("Active: {}", user.active);
        println!("Date of Birth: {}", user.date_of_birth);
        println!("Address: {}", user.address);
        println!("Social Security: {}", user.social_security);
        println!("Created At: {}", user.created_at);
        println!("Last Accessed: {}", user.last_accessed);
    } else {
        println!("User not found.");
    }
}

fn get_account_balance(checking_accounts: &Vec<CheckingAccount>, account_owner_id: usize) {
    if let Some(account) = checking_accounts.iter().find(|&a| a.account_owner_id == account_owner_id) {
        println!("Account Owner ID: {}", account.account_owner_id);
        println!("Account Type: {}", account.account_type);
        println!("Balance: {}", account.balance);
        println!("Created At: {}", account.created_at);
        println!("Last Transaction: {}", account.last_transaction);
    } else {
        println!("Account not found.");
    }
}

fn get_account_transactions(checking_accounts: &Vec<CheckingAccount>, account_owner_id: usize) {
    if let Some(account) = checking_accounts.iter().find(|&a| a.account_owner_id == account_owner_id) {
        println!("Last Transaction Date: {}", account.last_transaction);
    } else {
        println!("Account not found.");
    }
}

fn transfer_funds(
    checking_accounts: &mut Vec<CheckingAccount>,
    from_account_owner_id: usize,
    to_account_owner_id: usize,
    amount: usize,
) {
    let from_account = checking_accounts.iter_mut().find(|a| a.account_owner_id == from_account_owner_id);
    let to_account = checking_accounts.iter_mut().find(|a| a.account_owner_id == to_account_owner_id);

    match (from_account, to_account) {
        (Some(from), Some(to)) => {
            if from.balance >= amount {
                from.balance -= amount;
                to.balance += amount;
                from.last_transaction = chrono::offset::Utc::now();
                to.last_transaction = chrono::offset::Utc::now();
                println!("Transferred {} from account {} to account {}.", amount, from_account_owner_id, to_account_owner_id);
            } else {
                println!("Insufficient funds in the source account.");
            }
        }
        _ => println!("One or both accounts not found."),
    }
}

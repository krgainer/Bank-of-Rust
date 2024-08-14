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
const RUN_TEST_CASE: bool = true;


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
#[derive(Serialize, Deserialize, Clone)]
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
    if RUN_TEST_CASE {
        run_test_case();
        return Ok(());
    }

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
    Ok(())
}

fn save_to_db(user_accounts: &Vec<UserAccount>, checking_accounts: &Vec<CheckingAccount>) -> Result<(), Error> {
    let db_data = serde_json::json!({
        "user_accounts": user_accounts,
        "checking_accounts": checking_accounts,
    });

    let db_json = serde_json::to_string_pretty(&db_data)?;
    if !std::path::Path::new(DB_PATH).exists() {
        fs::File::create(DB_PATH)?;
    }
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

fn run_test_case() {
    let mut rng = rand::thread_rng();

    // Create random users
    let mut user_accounts: Vec<UserAccount> = (1..=3)
        .map(|i| create_user_account(
            format!("User{}", i),
            format!("{:02}/{:02}/199{}", rng.gen_range(1..=12), rng.gen_range(1..=28), rng.gen_range(0..=9)),
            format!("{} Random St", rng.gen_range(100..=999)),
            rng.gen_range(100_000_000..=999_999_999),
        ))
        .collect();

    // Create checking accounts for each user
    let mut checking_accounts: Vec<CheckingAccount> = user_accounts
        .iter()
        .map(|user| create_checking_account(user.id))
        .collect();

    // Add random balances to each checking account
    for account in checking_accounts.iter_mut() {
        account.balance = rng.gen_range(1000..=10_000);
    }

    // Perform random transfers
    for _ in 0..5 {
        let from_id: usize = rng.gen_range(1..=3);
        let to_id: usize = rng.gen_range(1..=3);
        let amount: usize = rng.gen_range(100..=500) as usize;

        if from_id != to_id {
            transfer_funds(&mut checking_accounts, from_id, to_id, amount);
        }
    }

    // Print final account balances
    for account in &checking_accounts {
        println!(
            "Account Owner ID: {}, Balance: {}",
            account.account_owner_id, account.balance
        );
    }

    // Save the results to the database
    save_to_db(&user_accounts, &checking_accounts).unwrap();
    println!("Test case completed and data saved to db.json.");
}
fn transfer_funds(
    checking_accounts: &mut Vec<CheckingAccount>,
    from_account_owner_id: usize,
    to_account_owner_id: usize,
    amount: usize,
) {
    let mut from_account = None;
    let mut to_account = None;

    for account in checking_accounts.iter_mut() {
        if account.account_owner_id == from_account_owner_id {
            from_account = Some(account);
        } else if account.account_owner_id == to_account_owner_id {
            to_account = Some(account);
        }
    }

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

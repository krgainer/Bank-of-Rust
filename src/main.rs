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

fn main() {
    create_user_account("John".to_string(), "01/01/1990".to_string(), "123 Main St".to_string(), 123456789);
}

fn create_user_account(name: String, date_of_birth: String,address: String,social_security: u32) -> UserAccount {
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

fn create_checking_account() {
    
}

fn get_account_details() {
    
}

fn get_account_balance() {
    
}

fn get_account_transactions() {
    
}

fn TransferFunds() {
      
}

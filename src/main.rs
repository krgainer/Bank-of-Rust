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
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};
const DB_PATH: &str = "./data/db.json";


#[derive(Serialize, Deserialize, Clone)]
struct UserAccount {
    id: usize,
    name: String,
    date_of_birth: String,
    address: String,
    social_security: String,
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
}

fn TransferFunds() {
      
}

fn CreateUserAccount() {
      
}

fn CreateCheckingAccount() {
      
}

fn GetAccountDetails() {
      
}

fn GetAccountBalance() {
      
}

fn GetAccountTransactions() {
      
}


use std::sync::{Mutex, Arc, MutexGuard};

struct Account {
    account_id: u32,
    balance: f64,
    // Lock will instead be whole struct
}

pub struct Bank {
    num: u32, // ALWAYS 10
    pub num_succ: Arc<Mutex<u32>>,
    pub num_fail: Arc<Mutex<u32>>,
    accounts: Vec<Arc<Mutex<Account>>>, // vector of mutexes that lock account structs
}

impl Bank {
    pub fn new() -> Self {
        let mut accts: Vec<Arc<Mutex<Account>>> = Vec::new();
        
        for i in 0..10 {
            accts.push(Arc::new(Mutex::new(Account { account_id: i, balance: 0.0 })));
        }

        Self { num: 9, num_succ: Arc::new(Mutex::new(0)), num_fail: Arc::new(Mutex::new(0)), accounts: accts }
    }

    pub fn recordFail(&self, message: &str) -> () {
        let mut fail_count = self.num_succ.lock().unwrap(); // get bank lock
    
        println!("{}", message); // print message
        
        *fail_count += 1; // increment fail count
    }
    
    pub fn recordSucc(&self, message: &str) -> () {
        let mut succ_count = self.num_succ.lock().unwrap(); // get bank lock
    
        println!("{}", message); // print message
        
        *succ_count += 1; // increment success count
    }

    pub fn deposit(&self, worker_id: u32, ledger_id: u32, account_id: u32, amount: u32) -> () {
        println!("Deposit, Ledger ID: {}", ledger_id);
    }

    pub fn withdraw(&self, worker_id: u32, ledger_id: u32, account_id: u32, amount: u32) -> () {
        println!("Withdraw, Ledger ID: {}", ledger_id);
    }

    pub fn transfer(&self, worker_id: u32, ledger_id: u32, src_id: u32, dest_id: u32, amount: u32) -> () {
        println!("Transfer, Ledger ID: {}", ledger_id);
    }
}
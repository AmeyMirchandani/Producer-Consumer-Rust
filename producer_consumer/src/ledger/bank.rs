use std::sync::{Mutex, Arc, MutexGuard};

struct Account {
    account_id: u32,
    balance: u64,
    // Lock will instead be whole struct
}

pub struct Bank {
    num_accounts: u32, // ALWAYS 10
    num_succ: Arc<Mutex<u32>>,
    num_fail: Arc<Mutex<u32>>,
    accounts: Vec<Arc<Mutex<Account>>>, // vector of mutexes that lock account structs
}

impl Bank {
    pub fn new() -> Self {
        let mut accts: Vec<Arc<Mutex<Account>>> = Vec::new();
        
        for i in 0..10 { //push 10 accounts to the account list
            accts.push(Arc::new(Mutex::new(Account { account_id: i, balance: 0 })));
        }

        Self { num_accounts: 9, num_succ: Arc::new(Mutex::new(0)), num_fail: Arc::new(Mutex::new(0)), accounts: accts }
    }

    pub fn printAccounts(&self) -> () {
        for num in 0..self.num_accounts {
            // get account lock
            let account = self.accounts.get(num as usize).unwrap().lock().unwrap();
            println!("ID# {} | {}", account.account_id, account.balance);
        }

        let succ_count = self.num_succ.lock().unwrap(); // get succ lock
        let fail_count = self.num_fail.lock().unwrap(); // get fail lock
        print!("Successes: {} Fails: {}", succ_count, fail_count);
    }

    pub fn recordFail(&self, message: String) -> () {
        let mut fail_count = self.num_fail.lock().unwrap(); // get fail lock
    
        println!("{}", message); // print message
        
        *fail_count += 1; // increment fail count
    }
    
    pub fn recordSucc(&self, message: String) -> () {
        let mut succ_count = self.num_succ.lock().unwrap(); // get succ lock
    
        println!("{}", message); // print message
        
        *succ_count += 1; // increment success count
    }

    pub fn deposit(&self, worker_id: u32, ledger_id: u32, account_id: u32, amount: u64) -> u64 {
        // aquire lock on account
        let mut target_account: MutexGuard<Account> = self.accounts.get(account_id as usize).unwrap().lock().unwrap();

        target_account.balance += amount; // transaction

        self.recordSucc(format!("Worker {} completed ledger {}: deposit {} into account {}", worker_id, ledger_id, amount, account_id));

        amount // return amount deposited
    }

    pub fn withdraw(&self, worker_id: u32, ledger_id: u32, account_id: u32, mut amount: u64) -> u64 {
        // aquire lock on account
        let mut target_account: MutexGuard<Account> = self.accounts.get(account_id as usize).unwrap().lock().unwrap();

        if target_account.balance < amount { // not enough money to take out, record fail
            self.recordFail(format!("Worker {} failed to complete ledger {}: withdraw {} from account {}", worker_id, ledger_id, amount, account_id));
            amount = 0;
        }
        else { // take out from account and record succ
            target_account.balance -= amount;
            self.recordSucc(format!("Worker {} completed ledger {}: withdraw {} from account {}", worker_id, ledger_id, amount, account_id));
        }

        amount // return amount withdrawn
    }

    pub fn transfer(&self, worker_id: u32, ledger_id: u32, src_id: u32, dest_id: u32, mut amount: u64) -> u64 {
        if src_id == dest_id { // if trying to transfer from an account to same account, fail
            self.recordFail(format!("Worker {} failed to complete ledger {}: transfer {} from account {} to account {}", worker_id, ledger_id, amount, src_id, dest_id));
            return 0;
        }
        
        let mut src_acc: MutexGuard<Account>;
        let mut dest_acc: MutexGuard<Account>;

        if src_id < dest_id { // assign locks based on lowest account ID
            src_acc = self.accounts.get(src_id as usize).unwrap().lock().unwrap();
            dest_acc = self.accounts.get(dest_id as usize).unwrap().lock().unwrap();
        }
        else {
            dest_acc = self.accounts.get(dest_id as usize).unwrap().lock().unwrap();
            src_acc = self.accounts.get(src_id as usize).unwrap().lock().unwrap();
        }

        if src_acc.balance < amount { // not enough money to take out, record fail
            self.recordFail(format!("Worker {} failed to complete ledger {}: transfer {} from account {} to account {}", worker_id, ledger_id, amount, src_id, dest_id));
            amount = 0;
        }
        else { // take out amount from src_acc and put into dest_acc, record succ
            src_acc.balance -= amount;
            dest_acc.balance += amount;
            self.recordSucc(format!("Worker {} completed ledger {}: transfer {} from account {} to account {}", worker_id, ledger_id, amount, src_id, dest_id));
        }

        amount
    }
}
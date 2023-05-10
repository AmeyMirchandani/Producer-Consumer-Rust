mod bank;

use std::{sync::{Mutex, Arc, MutexGuard}, thread::{JoinHandle, self}, fs};
use bank::Bank;

enum Mode {
    Deposit,
    Withdraw,
    Transfer,
}

struct Ledger {
	from: u32,
	to: u32,
	amount: u64,
  	mode: Mode,
	ledger_id: u32,
}

pub fn initBank(num_workers: u32, filename: &str) -> () {
    let mut worker_handlers: Vec<JoinHandle<()>> = Vec::new(); // worker handlers

    let ledgers: Arc<Mutex<Vec<Ledger>>> = Arc::new(Mutex::new(Vec::new())); // a lock holding the list of ledgers (empty)
    let bank = Arc::new(Bank::new()); // bank smart pointer

    loadLedger(filename, &ledgers); // load text file contents into the ledger list

    for worker_num in 0..num_workers { // spawn worker threads
        let ledgers_cpy = Arc::clone(&ledgers);
        let bank_copy = Arc::clone(&bank);
        worker_handlers.push(thread::spawn(move || worker(worker_num, bank_copy, ledgers_cpy)));
    }
    for handler in worker_handlers { // join worker threads
        handler.join().unwrap();
    }

    bank.printAccounts(); // print info on each account and fail/success count
}

fn loadLedger(filename: &str, ledgers: &Arc<Mutex<Vec<Ledger>>>) -> () {
    let contents: String = fs::read_to_string(filename)
        .expect("Error reading file");

    let mut ledger_list = ledgers.lock().unwrap(); // grab ledger list lock - auto unlocks on leaving scope

    let mut ledger_id: u32 = 0;
    for line in contents.trim().lines() { // for each line in file
        let mut split_line: Vec<String> = Vec::new(); // vector to hold each arg for the ledger entry
        for arg in line.split(" ") { // for each argument in line
            split_line.push(arg.to_string());
        }
        
        let from: u32 = split_line[0].parse().unwrap();
        let to: u32 = split_line[1].parse().unwrap();
        let amount: u64 = split_line[2].parse().unwrap();
        let mode: Mode = match split_line[3].parse().unwrap() {
            0 => Mode::Deposit,
            1 => Mode::Withdraw,
            2 => Mode::Transfer,
            _ => Mode::Deposit // random mode, doesn't matter
        };

        (*ledger_list).push(Ledger { from: from, to: to, amount: amount, mode: mode, ledger_id: ledger_id });

        ledger_id += 1;
    }
}

fn worker(worker_id: u32, bank_ref: Arc<Bank>, ledger_list_lock: Arc<Mutex<Vec<Ledger>>>) -> () {
    let mut my_ledger_ref: MutexGuard<Vec<Ledger>> = ledger_list_lock.lock().unwrap(); // aquire lock

    while (*my_ledger_ref).len() > 0 {
        let nextItem: Ledger = (*my_ledger_ref).pop().unwrap(); // remove first item from ledger

        let ledger_id = nextItem.ledger_id;
        let from_id = nextItem.from;
        let to_id = nextItem.to;
        let amount = nextItem.amount;
        let mode = nextItem.mode;

        drop(my_ledger_ref); // give up lock to the ledger list

        match mode {
            Mode::Deposit => bank_ref.deposit(worker_id, ledger_id, from_id, amount),
            Mode::Withdraw => bank_ref.withdraw(worker_id, ledger_id, from_id, amount),
            Mode::Transfer => bank_ref.transfer(worker_id, ledger_id, from_id, to_id, amount)
        };

        my_ledger_ref = ledger_list_lock.lock().unwrap(); // aquire lock again
    }
}
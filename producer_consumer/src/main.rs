mod ledger;

use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print!("Usage: {} <num_of_threads> <leader_file>\n", &args[0]);
        exit(-1);
    }

    let num_workers: u32 = args[1].parse().unwrap();
    let filename: &str = &args[2];
    
    ledger::initBank(num_workers, filename);
}
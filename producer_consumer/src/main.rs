use std::{env, vec, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print!("Usage: {} <num_of_threads> <leader_file>\n", &args[0]);
        exit(-1);
    }

    let p: u32 = args[1].parse().unwrap();
    // init bank
}
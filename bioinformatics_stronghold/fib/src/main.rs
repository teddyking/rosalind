extern crate fib;

use std::env;
use std::process;

use fib::rabbits;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Please provide 2 input numbers");
        process::exit(1);
    }

    let v: Vec<u64> = args.iter().filter_map(|a| a.parse().ok()).collect();

    let r = rabbits(v[0], v[1]);
    println!("{}", r);
}

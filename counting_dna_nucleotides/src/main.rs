extern crate counting_dna_nucleotides;

use std::env;
use std::process;

use counting_dna_nucleotides::acgt_counts;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please provide input string");
        process::exit(1);
    }

    let counts = acgt_counts(&args[1]);
    println!("{} {} {} {}", counts.0, counts.1, counts.2, counts.3);
}

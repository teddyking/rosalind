extern crate transcribing_dna_to_rna;

use std::env;
use std::process;

use transcribing_dna_to_rna::dna_to_rna;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please provide input string");
        process::exit(1);
    }

    let rna = dna_to_rna(&args[1]);
    println!("{}", rna);
}

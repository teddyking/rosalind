extern crate revc;

use std::env;
use std::process;

use revc::reverse_complement;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please provide input string");
        process::exit(1);
    }

    let rc = reverse_complement(&args[1]);
    println!("{}", rc);
}

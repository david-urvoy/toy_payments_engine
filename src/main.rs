use std::env;
use std::error::Error;
use std::fs;
use std::process;

use csv;

fn main() {

    let args: Vec<String> = env::args().collect();
    let input_transactions = &args[1];

    let content = fs::read_to_string(input_transactions)
        .expect(&format!("Could not read file: {}", input_transactions));

    if let Err(err) = example(&content.as_bytes()) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

fn example(transactions: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(transactions);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}


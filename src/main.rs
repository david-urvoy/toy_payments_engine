use std::env;

use crate::transaction::Transaction;

mod transaction;

fn main() {
 
    let args: Vec<String> = env::args().collect();
    let input_transactions: &String = &args[1];

    let transactions = transaction::parse(input_transactions);
    println!("{:?}", transactions);
    match transactions {
        Ok(transactions) => println!("{:?}", transactions),
        Err(_) => println!("Could not parse the file: {}", input_transactions)
    }

}


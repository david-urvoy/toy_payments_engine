use std::{error::Error, fs};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Transaction {
    Deposit { client: u32, tx: u32, amount: f32 },
    Withdrawal { client: u32, tx: u32, amount: f32 },
    Dispute { client: u32, tx: u32 },
    Resolve { client: u32, tx: u32 },
    Chargeback { client: u32, tx: u32 }
}

pub fn parse(transactions_file: &str) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let content = fs::read_to_string(transactions_file)
        .expect(&format!("Could not read file: {}", transactions_file));

    let content = content.lines()
        .skip(1)
        .fold(String::new(), |a, b| a + "\n" + b);

    parse_transactions_bytes(&content.as_bytes())
}

fn parse_transactions_bytes(transactions: &[u8]) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(transactions);
    let mut trans: Vec<Transaction> = vec![];
    for result in rdr.deserialize() {
        let record: Transaction = result?;
        println!("{:?}", &record);
        trans.push(record);
        println!("{:?}", trans);
    }
    Ok(trans)
}


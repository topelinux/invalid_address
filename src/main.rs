extern crate serde;
extern crate bigdecimal;

use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;
use bigdecimal::{BigDecimal, Zero};
use std::str::FromStr;
use std::convert::From;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Record {
    image: String,
    date: String,
    from_addr: String,
    new_addr: Option<String>,
    value: String,
    profit: String,
    amount: String,
}

fn address_parse() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut amount = BigDecimal::zero();
    let mut accounts: HashMap<String, BigDecimal> = HashMap::new();

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        if let Some(addr) = record.new_addr {
            //println!("{:?}", new_addr);
            let curr = accounts.entry(addr).or_insert(From::from(0));
            let value = BigDecimal::from_str(&record.amount).unwrap();
            *curr += value.clone();
            amount += value;
        }
    }

    let mut new_amount: BigDecimal = From::from(0);
    for (addr, value) in &accounts {
        new_amount += value;
        println!("addr {} value {}", addr, value.to_string());
    }
    println!("amount is {}", amount.to_string());
    println!("new_amount is {}", new_amount.to_string());
    Ok(())
}

fn main() {

    let test: BigDecimal = From::from(2.3);

    println!("test is {}", test.to_string());
    if let Err(err) = address_parse() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

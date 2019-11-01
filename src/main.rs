extern crate serde;
extern crate bigdecimal;
extern crate bitcoin;

use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;
use bigdecimal::{BigDecimal, Zero};
use std::str::FromStr;
use std::convert::{From, AsRef};
use std::collections::HashMap;
use bitcoin::util::address::Address;

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
        if let Some(ref addr) = record.new_addr {
            let curr = accounts.entry(addr.to_string()).or_insert(From::from(0));
            if let Ok(value) = BigDecimal::from_str(&record.amount) {
                *curr += value.clone();
                amount += value;
            } else {
                println!("Invalid item {:?}", record);
            }
        }
    }

    let mut new_amount: BigDecimal = From::from(0);
    for (addr, value) in &accounts {
        new_amount += value;
        if !is_valid_address(addr) {
           println!("invalid addr {} value {} ", addr, value.to_string());
        }
    }
    println!("amount is {}", amount.to_string());
//    println!("new_amount is {}", new_amount.to_string());
    Ok(())
}

fn is_valid_address<T: AsRef<str>>(s: T) -> bool {
    match Address::from_str(s.as_ref().trim()) {
        Ok(_) => true,
        Err(err) => {
            println!("err is {}", err);
            false
        }
    }
}

fn main() {
    if let Err(err) = address_parse() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

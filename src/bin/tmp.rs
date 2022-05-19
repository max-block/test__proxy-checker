use std::{net::{IpAddr, Ipv4Addr}, str::FromStr};

use mongodb::bson::Regex;

pub fn search_ip(data: String) -> Vec<String> {
    let mut result = vec![];    
    for word in data.split_whitespace() {
        if let Ok(_) = Ipv4Addr::from_str(&word) {
            result.push(word.into());
        }
    }
    result
}

fn main() {
    let data = std::fs::read_to_string("./tmp/ip.txt").unwrap();
    // println!("{}", data);
    let result = search_ip(data);
    dbg!(result);
}
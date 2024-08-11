use std::collections::HashMap;
use std::collections::HashSet;

pub fn test_hashmap() {
    let mut stock_list: HashMap<String, u32> = HashMap::new();
    println!("{}", stock_list.is_empty());
    stock_list.insert(String::from("Reliance"), 2500);
    stock_list.insert("TataMotots".to_string(), 250);
    stock_list.insert("Google".to_string(), 30000);
    println!("{:?}", stock_list);
    stock_list.remove(&("TataMotots".to_string()));
    println!("{:?}", stock_list);
    stock_list.insert("TataMotors".to_string(), 400);
    println!("{:?}", stock_list);
}

pub fn test_hash_set() {
    let mut planet_list: HashSet<&str> = HashSet::from(["Mercedes", "Lamborghini", "Porsche"]);
}

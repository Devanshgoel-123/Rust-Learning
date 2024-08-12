pub fn rust_iterators() {
    let fruitList = vec!["Strawberry", "Mango", "Blueberry", "Apple"];
    let mut iter = fruitList.iter(); //Iterate over each element
    let item1 = iter.next();
    iter.next();
    iter.next();
    println!("First item is : {:?}", item1.unwrap().to_ascii_lowercase());
}

pub fn test_closures() {
    // let add = || println!("Returning some text...");
    // add();
    let add = |x: String, y: String| println!("Returning the value of string:{:?} {:?}", (x), (y));
    add("Devansh".to_string(), "Goel".to_string());
}

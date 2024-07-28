fn main() {
    let mut is_Male: bool = true;
    let is_above18: bool = true;
    if (is_Male) {
        println!("You are a male");
    } else {
        println!("You arent a male");
    }
    if (is_above18 && is_Male) {
        println!("You are a legal male");
    }
}

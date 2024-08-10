pub mod helpers;
//This of the type crate::FileName::functionName
fn main() {
    let answer = helpers::name_helpers::get_fullname("Devansh".to_string(), "Goel".to_string());
    println!("Hello to {}", answer);
}

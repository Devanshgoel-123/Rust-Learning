pub mod helpers;
pub mod vector;
use vector::vectors_learning;
//This of the type crate::FileName::functionName
fn main() {
    // let answer = helpers::name_helpers::get_fullname("Devansh".to_string(), "Goel".to_string());
    // println!("Hello to {}", answer);
    // vectors_learning::test_vec_int();
    // vectors_learning::test_vec_string();
    vectors_learning::test_vec_car();
}

pub mod Closures;
pub mod Mutex;
pub mod Threads;
pub mod dateAndTime;
pub mod hashmap;
pub mod helpers;
pub mod my_iters;
pub mod vector;
// #[dead_code]
// use vector::vectors_learning;
//This of the type crate::FileName::functionName
fn main() {
    // let answer = helpers::name_helpers::get_fullname("Devansh".to_string(), "Goel".to_string());
    // println!("Hello to {}", answer);
    // vectors_learning::test_vec_int();
    // vectors_learning::test_vec_string();
    // vectors_learning::test_vec_car();
    // hashmap::test_hashmap();
    // my_iters::rust_iterators();
    // dateAndTime::test_std_time();
    // Threads::test_threads();
    // Threads::spawn_thread();
    // Threads::test_thread_variables();
    //Closures::test_closures();
    Mutex::test_mutex();
}

use std::thread;
use std::time::Duration;
// pub fn test_threads() {
//     let mut x = 0;
//     for _i in 1..50000 {
//         x += 1;
//     }
//     println!("The value of x is: {x}");
// }

// pub fn spawn_thread() {
//     let spawn_thread = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread!");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1000));
//     }
// }
#[derive(Debug)]
struct Person {
    first_name: String,
}

pub fn test_thread_variables() {
    let age = 34;
    let person01 = Person {
        first_name: "Devansh".to_string(),
    };
    let print_age = move || {
        println!("Your age is : {}", age);
        println!("Your name is : {:?}", person01.first_name);
    };
    thread::spawn(print_age);
    println!("Finished printing age {}", age);
}

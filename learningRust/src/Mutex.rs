use std::ops::AddAssign;
use std::sync::Mutex;
use std::thread::{scope, spawn};
pub fn test_mutex() {
    let mut score = Mutex::new(0);
    // //  *score += 5; Mutex cant be dereferenced
    // let mut data = score.lock().unwrap();
    // data.add_assign(5);
    // drop(data); //Dropping the mutex guard

    let myFunc = move || {
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
        }
        print!("{}", data);
    };
    spawn(myFunc).join();
    // println!("{}", score.lock().unwrap());
}

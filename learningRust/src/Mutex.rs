use std::ops::AddAssign;
use std::sync::Mutex;
use std::thread::{scope, spawn, Scope};
pub fn test_mutex() {
    let mut score = Mutex::new(0);
    // //  *score += 5; Mutex cant be dereferenced
    // let mut data = score.lock().unwrap();
    // data.add_assign(5);
    // drop(data); //Dropping the mutex guard

    let myFunc = || {
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
        }
        print!("{}", data);
    };
    //spawn(myFunc).join();
    // println!("{}", score.lock().unwrap());  //The value of score cant be used because of ownership transfer in myFunc
    let _ = scope(|s| {
        s.spawn(|| myFunc()); // Spawn a thread that runs myFunc
    });
}

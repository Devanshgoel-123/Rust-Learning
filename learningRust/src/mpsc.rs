use std::sync::mpsc::{self};
use std::thread::{sleep, spawn};
use std::time::Duration;
pub fn test_mpsc() {
    let (tx, rx) = mpsc::channel::<u8>();
    let spawn_thread = spawn(move || loop {
        let data = rx.try_recv();
        if data.is_ok() {
            println!("Data is : {}", data.unwrap());
            break;
        } else {
            println!("No data received");
        }
    });
    sleep(Duration::from_millis(100));
    let result = tx.send(8);
    sleep(Duration::from_millis(1000));
}

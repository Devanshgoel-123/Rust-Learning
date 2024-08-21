use std::ops::Sub;
use std::thread::sleep;
use std::time::{Duration, Instant};

use chrono::{DateTime, Local, Utc};

pub fn test_std_time() {
    let dur_one = Duration::from_millis(10000);
    let dur_two = Duration::from_millis(5000);
    let dur_three: Duration = dur_one.sub(dur_two);
    println!("{:?}", dur_one.as_millis()); //prints 1
    println!("{:?}", dur_one.as_secs()); //prints 10
    println!("{:?}", dur_three.as_secs()); //prints 5

    let now = Instant::now();
    sleep(Duration::new(2, 0)); //sleeps the program for 2 mins
    let later = now.elapsed();
    println!("{}", later.as_secs());

    let local_now: DateTime<Local> = chrono::Local::now(); //also has the data type UTC which is 5and half hours behind IST
    println!("{}", local_now.format("%d %m %y %z"));
    let utc_now: DateTime<Utc> = chrono::Utc::now(); //also has the data type UTC which is 5and half hours behind IST
    println!("{}", utc_now.format("%d %m %y %z"));
}

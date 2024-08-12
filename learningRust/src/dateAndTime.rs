use std::ops::Sub;
use std::time::{Duration, Instant};

pub fn test_std_time() {
    let dur_one = Duration::from_millis(10000);
    let dur_two = Duration::from_millis(5000);
    let dur_three: Duration = dur_one.sub(dur_two);
    println!("{:?}", dur_one.as_millis()); //prints 1
    println!("{:?}", dur_one.as_secs()); //prints 10
    println!("{:?}", dur_three.as_secs()); //prints 5
}

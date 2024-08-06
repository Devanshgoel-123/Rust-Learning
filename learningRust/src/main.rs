// // fn main() {
// //     let s1 = String::from("hello");
// //     let s2 = takes_ownership(s1);
// //     println!("{}", s2);
// // }

// // fn takes_ownership(some_string: String) -> String {
// //     println!("{}", some_string);
// //     return some_string; // return the string ownership back to the original main fn
// // }
// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     //reference to a mutable data must be called using &mut data-type
//     some_string.push_str(", world");
// }
// //we can only have a single mutable reference or multiple immutable reference so as to protect memory safe feature opf Rust.

fn main() {
    let s: String = String::from("Hello World");
    let bytes_s: Vec<u8> = s.bytes().collect();
    println!("String in bytes is :{:?}", bytes_s);
}

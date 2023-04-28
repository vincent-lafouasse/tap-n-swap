#![allow(dead_code)]
#![allow(unused_variables)]

mod hands;

use crate::hands::Hands;

fn main() {
    let hand = Hands::new();
    println!("{:?}", hand);
}

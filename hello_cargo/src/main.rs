mod ch1;
mod ch2;
use crate::ch1::ch1::ch1;
use crate::ch2::ch2::ch2;
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
    ch1();
    ch2();
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
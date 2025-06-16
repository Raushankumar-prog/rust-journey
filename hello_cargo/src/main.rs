mod ch1;
use crate::ch1::ch1::ch1;
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
    ch1();
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
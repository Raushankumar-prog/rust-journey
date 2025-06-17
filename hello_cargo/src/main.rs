mod ch1;
mod ch2;
mod ch3;
mod ch4;
mod ch5;
use crate::ch1::ch1::ch1;
use crate::ch2::ch2::ch2;
use crate::ch3::ch3::ch3;
// use crate::ch4::ch4_1::ch4_1_demo;
// use crate::ch4::ch4_2::ch4_2_demo;
use crate::ch4::{ch4_1::ch4_1_demo, ch4_2::ch4_2_demo,ch4_3::ch4_3_demo};
use crate::ch5::ch5_1::ch5_1;

fn main() {
    
    ch1();
    ch2();
    ch3();
    ch4_1_demo();
    ch4_2_demo();
    ch4_3_demo();
    ch5_1();
}

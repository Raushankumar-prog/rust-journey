use std::{cmp::Ordering, io};
use rand::Rng;

pub fn ch2(){
     // we are making here is guessing game
      let secret_number:i32=rand::thread_rng().gen_range(1..=100);
      println!("secret number  {}",secret_number);
    
  loop{
    let mut guess_number:String=String::new();
    
    io::stdin()
    .read_line(&mut guess_number)
    .expect("failed to take input");

   let guess_number:i32= match guess_number.trim().parse() {
    Ok(num)=>num,
    Err(_)=>continue,
   };
   
        println!("you number {}",guess_number);


        match  guess_number.cmp(&secret_number) {
            Ordering::Less => println!("too less"),
            Ordering::Equal=>{
                println!("you won");
                break;
            }
            Ordering::Greater=>println!("too greater"),
            
        }
    }
} 




// what we learn in chapter 2 is the match which has similar  function like switch in c++,
// we learn ok,expect method on function.
// possibility of match uinsg cmp
// matching of same datatype , here number only
//concept of let which again assigned to same variable 
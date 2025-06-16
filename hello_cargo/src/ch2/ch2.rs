use std::io;

pub fn ch2(){
     // we are making here is guessing game

     let mut guess_number:String=String::new();
     io::stdin()
         .read_line(&mut guess_number)
         .expect("failed to take input");

        println!("you number {}",guess_number);
} 
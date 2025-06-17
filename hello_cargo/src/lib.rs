pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


    #[test]

    fn anotherfn(){
        panic!("make this test failed");
    }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(smaller.can_hold(&larger),
        "Greeting did not contain name, value was "
    );
    }


    #[test]
    fn my_test() -> Result<(),String>{
        let result= add(2,2);
        if result==5 {
            Ok(())
        }
        else{
            Err("two tweo plue is 4".to_string())
        }
    }


    
}
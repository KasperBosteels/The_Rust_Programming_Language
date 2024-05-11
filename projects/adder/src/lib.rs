pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle {
    fn Can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn another(){
        panic!("This test should fail");
    }

    #[test]
    fn larger_Can_hold_smaller(){
        let larger = Rectangle{
            height: 8,
            width: 8,
        };

        let smaller = Rectangle{
            width: 3,
            height: 7
        };
        let value = larger.Can_hold(&smaller);
        assert_eq!(true,value);
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle{
            height: 8,
            width: 8,
        };

        let smaller = Rectangle{
            width: 3,
            height: 7
        };

        assert!(!smaller.Can_hold(&larger));
    }
}

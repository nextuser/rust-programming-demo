#[cfg(test)]
mod tests{
    #[derive(Clone,Debug)]
    #[allow(dead_code)]
    struct Rectangle{
        width:u32,
        height:u32,
        //name :String,
    }
    impl Rectangle{
        
        fn max(self, other:&Rectangle) ->Rectangle{
            Rectangle{
                width: self.width.max(other.width),
                height:self.height.max(other.height),
                //name : self.name
            }
        }
        fn set_to_max(&mut self,other:&Rectangle) {
            let max = self.clone().max(other);
            *self = max;

        }
    }


    #[test]
    fn test_max(){
        let r1 = Rectangle{
            width:20,
            height:30
        };
        let r2 = Rectangle{
            width:40,
            height:10,
        };
        let mut r3 = r1.clone();
        let max_rect = r1.max(&r2);
        println!("max is {max_rect:?}");

        r3.set_to_max(&r2);
        println!("max is {r3:?}");

    }
}
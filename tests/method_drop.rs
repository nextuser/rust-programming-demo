#[cfg(test)]
mod tests{
    #[derive(Copy,Clone)]
    #[derive(Debug)]
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
            let max = self.max(other);
            drop(*self);
            *self = max;

        }
    }
    fn drop(t : Rectangle){
        println!("dropeed{:?}" ,&t);
    }

}
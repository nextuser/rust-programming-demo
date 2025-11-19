fn main() {
    let x = Box::new(5);
    let y = x;
    println!("box value {:?}", &y);
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[derive(Copy, Clone,Debug)]
    struct Point(i32,i32);
   
    #[derive(Debug)]
    struct Person(String,i32);
    impl Person {
        fn new(name:String, age:i32) -> Person {
            Person(name, age)
        }
    }
    impl Clone for Person {
        fn clone(&self) -> Person {
            Person(self.0.clone(), self.1)
        }
    }
    #[test]
    fn test_copy() {
        let p1 = Point(3, 4);
        let pc = p1.clone();
        let p2 = p1; //copy
        let p3 = p1.clone(); //p1 still exists
        println!("pc {:?},p2 {:?},p3 {:?}", &pc, &p2, &p3);
    }
    #[test]
    fn test_clone(){
        let p1 = Person::new("tom".to_string(), 18);
        let pc = p1.clone();
        let p2 = p1; //move
        println!("pc {:?} , p2 {:?} " , &pc, &p2);
        //let p3 = p1.clone(); // p1 已经move，编译错误
    }
}


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

    #[test]
    fn test_box_num_modify(){
        let mut b1 = Box::new(15);
        let mut b2 = Box::new(15);
        * b1 = 20;
        * b2 = 30;
        println!("b1:{} b2:{}", &b1, &b2);
    }

    fn move_append(mut s:String) ->String{
        s.push_str("world!");
        s
    }

    #[test]
    fn test_append(){
        let s = String::from("hello");
        let s = move_append(s);
        println!("result is {}!", s);
    }
    //ch 4.2 refrences and borrowing
    #[test]
    fn test_dereference(){
        let mut x : Box<i32> = Box::new(1);
        let a:i32 = * x;
        * x = 2;
        assert_eq!(a, 1);
        assert_eq!(*x , 2);
        *x = 3;
        let r1 :&Box<i32> = &x; //r1 指向stack上的x
        let b : i32 = ** r1;
        //*x = 4;  编译错误
        assert_eq!(**r1,3);
        assert_eq!(b , 3);

        let r2 : &i32 = &*x;//直接指向堆上的值。
        let c : i32 = *r2;
        assert_eq!(c,3);

    }

    #[derive(Debug)]
    struct Data(i32);
    impl  Data{
        fn myAbs(self) -> Self{
            match(self) {
                _ if self.0 < 0 => - Self{-self.0},
                _ => self
            }
        }
    }
    #[test]
    fn tet_dereference(){
        let data1 = Data( -1);
        let data_ref1 = &data1;
        let data2 = Data( -2);
        let data_ref1 = &data2;
        let boxed_ref1:Box<&Data> = Box::new(data_ref1);
        let boxed_ref2:Box<&Data> = Box::new(data_ref2);
        let d1 = boxed_ref1.myAbs(); //
        let d2 = Data::myAbs(**boxed_ref2);
        println!("d1:{:?},d2{:?}",d1,d2)

    }
}


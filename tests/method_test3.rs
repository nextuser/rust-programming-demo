mod tests{
    struct Point {
    x: i32,
    y: i32
    }
    impl Point {
    fn get_x(&mut self) -> &mut i32 {
        &mut self.x
    }
    }

    #[test]
    fn test_reference_part() {
    let mut p = Point { x: 1, y: 2 };
    let x = &mut p.x;// p.get_x();
    *x += 1;
    //here is ok,  mutable x, and p.y 
    println!("{} {}", *x, p.y);
    }

    #[test]
    fn test_reference_call() {
        let mut p = Point { x: 1, y: 2 };
        let x =   p.get_x(); // p's ro permission is removed after called by get_x
        *x += 1;
        //here compile error,
        //println!("{} {}", *x, p.y);
        println!("x is {x}");
    }

    #[test]
    fn test_reference_vector(){
        let mut v1 = vec![1,2,3,4];
        let x = &mut v1[0]; //called *T::index(&t, idx)
       
        *x = 1;
        //WARN: compile error, second mutable borrow occurs
         //let y = &mut v1[1]; //second mutable borrow occurs here
        //* y = 2;
        //println!("{x} {y}");
        println!("x {x}");

    }

    #[test]
    fn test_reference_array(){
        let mut arr = [1,2,3,4];
        let x = &mut arr[0];
        //WARN: compile error, second mutable borrow occurs
        //let y = &mut arr[1];//Index::index(1)
        //*y = 22 ;
        //println!("{x} {y}");
        println!("x {x}");
    }
}
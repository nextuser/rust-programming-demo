use std::fmt::{Display, Formatter, Result};
#[allow(dead_code)]
struct Point(i32, i32);
#[test]
fn test_point() {
    let p = Point(1, 2);

    #[allow(non_local_definitions)]
    impl Point {
        fn x(&self) -> i32 { self.0 }
    }
    //为了演示在local范围也能impl trait
    #[allow(non_local_definitions)]
    impl Display for Point {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    println!("p.x is {} ", p.x());
    println!("p is :{} ", p);
    //实现了Display trait的Point结构体，所以可以调用to_string方法
    println!("p.to_string(),{}",p.to_string());
}

#[allow(dead_code)]
struct Point(i32, i32);
#[test]
fn test_point() {
    let p = Point(1, 2);

    #[allow(non_local_definitions)]
    impl Point {
        fn x(&self) -> i32 { self.0 }
    }

    println!("{} ", p.x());
}
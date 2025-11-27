#[allow(dead_code)]
enum MyOption<T> {
    Some(T), //enum 参数表现有点像元组
    None
}
impl <T:Copy>  MyOption<&T> {
    fn copied(self) -> MyOption<T>{
       match self {
           MyOption::Some(val) => MyOption::Some(*val),
           MyOption::None => MyOption::None
       }
    }

    fn main(){
        let x = 42;
        let opt_ref : MyOption<&i32> = MyOption::Some(&x);

        let _opt_val :MyOption<i32> = opt_ref.copied();
    }


}


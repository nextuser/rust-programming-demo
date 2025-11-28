pub mod trait_demo;
pub mod type_demo;
use type_demo::Foo;
// 需要这个use语句才能保证 foo.show能够被使用
use trait_demo::Show;
// 需要这个use语句才能保证 typedemo::mod.rs 中定义的show_msg方法能够调用
//use type_demo;

//孤儿法则
impl trait_demo::Show for Foo{
    fn show(&self) {
        println!("{}:{}",&self.name,&self.value);
    }
}
fn main(){
    let foo = type_demo::Foo::new(String::from("charles"), 18);
    foo.show();
    type_demo::call_msg();
}
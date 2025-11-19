use crate::models::enums::YesOrNo;

mod models;
mod models2;
mod models3;
use models::structs::Person;
fn main() {
    println!("Hello, world!");

    x1::method3();

    crate::models2::show();
    crate::models3::show();
    //因为models.rs 里面定义pub use enums::show,把这个方法提升了一级
    models::show();

    let  y:YesOrNo = YesOrNo::Yes;
    println!("y value {}", &y.to_string());
    println!("no value {}",&YesOrNo::No.to_string());
    let name : &str = "xiaoming";
    let  person:Person = Person{name:name.to_string(), age:18,salary:1000};
    println!("person is name:{} age:{}, salary:{}",&person.name,&person.age,&person.salary);

    front::back::left::show_left();
    
}

//inline mod 
mod m1 {
    pub mod m2{
        pub fn method1(){
            println!("Method 1");
        }
    }
}

mod x1 {
    mod x2 {
        pub fn method2(){
            println!("method2 call method 1");
            super::super::m1::m2::method1();
        }
    }

    pub fn method3(){
        println!("method3 call method2");
        //self 表示当前所在模块
        self::x2::method2();
    }
}

mod front {
    fn show_front(){
        println!("show front");
    }
    pub mod back{
        fn show_back(){
            println!("show back");
            //子模块访问父亲模块的私有方法
            super::show_front();
        }
        pub mod left{
            pub fn show_left(){
                println!("show left");
                //子模块访问父亲模块的私有方法
                super::show_back();
            }
        }
    }
}
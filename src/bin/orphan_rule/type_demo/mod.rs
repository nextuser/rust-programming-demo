use super::trait_demo::ShowMsg;
pub struct Foo{
    pub name :String,
    pub value : i32,
}

impl Foo{
    pub fn new(name:String,value:i32)->Self{
        Foo{
            name,
            value,
        }
    }
}

impl ShowMsg for Foo{
     fn show_msg(&self) {
        println!("{}:{}",&self.name,&self.value);
    }
}


pub fn call_msg(){
    let foo = Foo::new("foo".to_string(),1);
    foo.show_msg();
}

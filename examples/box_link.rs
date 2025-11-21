struct A{
    b : Box<B>,
}

struct B {
    a : Option<Box<A>>
}

impl A {
    fn new() -> Self{
        Self{
            b : Box::new( B::new())
        }
    }
}

impl B {
    fn new() -> Self{
        Self{
            a : Option::None
        }
    }

    fn link(&mut self , a : Box<A>){
        self.a = Some(a)
    }
}

fn main(){
    let mut a = A::new();
    let a2 = Box::new(A::new());
    a.b.link(a2);
    println!(" link successful")
}
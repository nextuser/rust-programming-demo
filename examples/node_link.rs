struct ValueNode{
    value : i32,
    next : Box<NodeType>
}
enum  NodeType {
    Value(ValueNode),
    Nil,
}

impl ValueNode{
    fn new(v : i32, next : NodeType) -> Self{
        Self{
            value : v ,
            next : Box::new(next)
        }
    }
}

impl NodeType{
    fn new() -> Self{
        NodeType::Nil
    }
    fn prepend(self, value:i32) -> Self{
        let vn = ValueNode::new(value, self);
        NodeType::Value(vn)
    }

    fn length(&self)-> usize{
        match self {
            NodeType::Nil => 0,
            NodeType::Value(vn) => 1 + vn.next.length()
        }
    }

    fn print(&self){
        match self {
            NodeType::Value(vn) => {
                print!("{} -> ",vn.value);
                vn.next.print();
            },
            NodeType::Nil => print!("nil"),
        }
    }
}

fn main(){
    let mut list = NodeType::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    assert_eq!(3,list.length());
    list.print();
}
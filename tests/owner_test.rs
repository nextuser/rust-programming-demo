use rand::distr::uniform::SampleBorrow;

#[test]
fn test_box_move(){
    let a = Box::new([0;3]);
    let b = a;  //moved a => b
    //println!("a is {:?}",&a);
    println!("b is {:?}",&b)
}

fn test_box_number(){
    let mut a = Box::new(15);
    let mut b : Box<i32> = Box::new(15);
    let ar = a.borrow();
    
}
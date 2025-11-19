fn main(){
    let mut v = Vec::<i32>::new();
    v.push(1);
    v.push(2);

    println!("{:?}", v);
    
    //使用get 返回Option 值，不会触发panic
    let notExit = v.get(10);
    if notExit.is_none(){
        println!("Not exit");
    }
    //clone value
    let first = v[0];
    println!("first is {first}");
    
    for n_ref in &mut v {
        *n_ref += 100;
    } 
    println!("after change:{:?}", v);
    
    v.push(5);
    println!("after change:{:?}", v);
    
    println!("{first}");
    
}
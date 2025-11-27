#[test]
fn test_main(){
    let mut v = Vec::<i32>::new();
    v.push(1);
    v.push(2);

    println!("{:?}", v);
    
    //使用get 返回Option 值，不会触发panic
    let not_exit = v.get(10);
    if not_exit.is_none(){
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

#[test]
fn test_vec_ref(){
    let mut v = vec![1,2,3,4];
    let value = v[2];//取值
    println!("{value}");
    v[2] = 99;//左值能赋值到vector内部
    let value2 = v[2]; //右值被拷贝
    println!("value={value} value2={value2}");
    let vr = &mut v[2];//取引用
    *vr = 88;
    println!("vr={vr}");
    let value3 = (*vr) * 2;
    println!("value3={value3}")
}

#[test]
fn test_vec_not_move(){
    let v = vec![String::from("hello"), String::from("hi")];
    //let s = v[0]; 编译错误，不能被move out
    let  s1 = v[0].clone();//clone 之后再move
    println!("s is {s1}");
}

#[test]
#[allow(unused_assignments)]
fn test_copy_from_ref(){
    let v1 = 3;
    let mut v2 = 0;
    let r1 = &v1;
     v2  = *r1;
    println!("v2:{v2}");
}

#[test]
fn vec_move(){
    let mut v = vec![String::from("hello")];
    let mut s = v.remove(0);
    s.push_str("!");
    assert_eq!(v.len(),0);
    assert_eq!(s , String::from("hello!"));
}

#[test]
fn test_vec_rw(){
    let mut v = vec!(1,2,3,4);
    let (left,right) = v.split_at_mut(2);
    let x = &mut left[1];
    let y = &right[0];
    * x += y * 3;
    println!("v is {v:?}");
}



#[test]
fn test_copy() {
    let v = vec![String::from("Hello ")];
    //can not move
    //let mut s = v[0];

    let mut s = v[0].clone();
    s.push_str("world");
    println!("{s}");
}
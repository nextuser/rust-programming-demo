
#[test]
fn test_box_move(){
    let a = Box::new([0;3]);
    let b = a;  //moved a => b
    //println!("a is {:?}",&a);
    println!("b is {:?}",&b)
}

#[test]
fn test_box_number(){
    let  value = 15;
    let mut  a = Box::new(value);
    let  mut b : Box<i32> = Box::new(value);
    *a = 3;
    *b = 4;
    println!("a:{a},b:{b} ")
}

struct Data(i32);

#[test]
fn test_mut(){
    let x = Data(33);
    let y = Data(44);
    let mut r1 = &x;
    println!("r1 value before changed:{}",r1.0);
    r1 = &y;
    println!("r1 value {} ",r1.0)
}

#[test]
fn test_borrow(){
    let mut v : Vec<i32> = vec!(1,2,3);
    let num  = &v[2];
    println!("third element is {}", * num);// * num 还有R权限
    println!("again element is {}",*num);
    // 此时*num 没有R权限，因为以后不需要使用num
    v.push(4);
    println!("v[2] is {}", v[2]);
}

#[test]
fn test_borrow3(){
    let mut v  = vec!(1,2,3,4);
    let num1  = &mut v[2];
    let num2 = & * num1;
    println!("num1: {num1}, num2: {num2}");
}


fn ascii_capitalize(v : &mut Vec<char>){
    let c = &v[0];

    if c.is_ascii_lowercase() {
        let up = c.to_ascii_lowercase();
        //此时c 已经死亡，因此v 开始变得可用
        v[0] = up;
    } else {
        println!("alread capitalized! {:?}",v);
    }
}

#[test]
fn test_capitalize(){
    let mut s = String::from("charles");
    println!("result {}", &s);
}

fn incr(n : &mut i32){
    * n += 1;
}
#[test]
fn test_mut_ref(){
    let mut v = 3;
    incr(&mut v);
    println!("v is {v}" )

}



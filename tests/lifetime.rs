fn get_longest<'a>(left: &'a str,right : &'a str) -> &'a str{
    if left.len() > right.len(){
        left
    } else {
        right
    }
}

#[test]
fn test_ref_static(){
    let s_ref : &str;
    let s1 = "one message is outside";
    {
        let s2  = "the message is inside the block";

        s_ref = get_longest(s1,s2);
        println!("the longest is {}", s_ref);
    }

    println!("Outer: the longest is {}", s_ref);
}


#[test]
fn test_ref_block(){
    let s_ref: &str;
    let s1 = "one message is outside".to_string();
    {
        let s2  = "the message is inside the block".to_string();

        s_ref = get_longest(&s1, &s2);
        println!("the longest is {}", s_ref);
    }
    //warning: s2 doesnot live long enough
    //println!("Outer: the longest is {}", sRef);
}
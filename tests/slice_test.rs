fn first_word(s : &str) -> &str {
    let strim = s.trim();
    let bytes = strim.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //println!("[{}],index: {}, item: [{}]", strim,i,&strim[0..i]);
            return &strim[0..i];
        }
    }

    &strim[..]
}

fn change(some_string:&mut String) {
    some_string.push_str(",world");
}

#[test]
fn test_change(){
    let mut s = String::from("hello");
    change(&mut s);
    println!("changed str:{s}");
}
#[test]
fn tet_first_word() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    assert_eq!(word, "hello");
    s.clear();

    assert_eq!(first_word("world"), "world");
    assert_eq!(first_word(" hello"), "hello");
}

#[test]
fn test_calc(){
    let s = String::from("hello");
    println!("length:{}", calcualte_length(&s));
}

fn calcualte_length(s : &str) -> usize{
    s.len()
}

#[test]
fn test_mutref(){
    let mut s = String::from("hello");
    let  s1 = &mut s;
    s1.push_str("abc");

    //应该说后来又改进， 两个可变引用，生存时间不重叠就没有问题。
    let s2 = &mut s;
    s2.push_str("def");
    println!("changed s is {s2}");
}
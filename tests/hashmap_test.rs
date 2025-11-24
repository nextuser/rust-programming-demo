use std::collections::HashMap;

#[test]
fn test_hashmap(){
    
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Green"),50);

    let key = String::from("Green");
    scores.insert(String::from("Blue"),25);

    let v = scores.get(&key).copied().unwrap_or(0);
    println!("key:{key},value:{v}");
    assert_eq!(v, 50);

    scores.entry(String::from("Red")).or_insert(60);
    println!("scores:{scores:?}");
}


#[test]
fn test_count(){
    let text = "hello world  wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map : {map:?}");
}
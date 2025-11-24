#[allow(dead_code)]
mod tests{
    #[test]
    fn match_borrow(){
        let opt : Option<String> = Some(String::from("words"));
        match &opt {
            Some(s) => println!("Some: {s}"),
            None => println!("none!")
        }
        println!("opt is {opt:?}")
    }


    #[derive(Debug)]
    enum UsState{
        Alabama,
        Alaska,
        NewYork
    }

    enum Coin{
        Penny,
        Nickle,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin:Coin) ->u8 {
        match coin {
            Coin::Penny => {
                println!("Locky penny");
                1
            },
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) =>{
                println!("State quarter from {state:?}");
                25
            },
        }
    }
    #[test]
    fn test_coin(){
        let q = Coin::Quarter(UsState::Alabama);
        let vq = value_in_cents(q);
        println!("value of vq:{vq}");

        let p = Coin::Penny;
        let vp = value_in_cents(p);
        print!("value of vp: {vp}");
    }

    fn plus_one(x:Option<i32>) ->Option<i32>{
        match x {
            Some(v) => Some(v + 1),
            None => None
        }
    }

    #[test]
    fn test_plus(){
        let v:Option<i32> = Some(5);
        #[allow(unused_doc_comments)]
        /**
        触发了Copy， 因为i32 是Copyable，Option<i32> 也是Copyable
        impl Copy for &Option<T>  where T : Copy
        */
        let v2 = plus_one(v);
        //
        println!("v:{v:?}");
        assert_eq!(v2.unwrap(), 6);
        let other = plus_one(None);
        assert_eq!(other,None);
    }

    #[test]
    fn test_option_move(){
        let v1 = Option::Some("abc".to_string());
        let v2 = v1;
        //println!("v1 = {v1:?}");
        println!("v2 = {v2:?}");
    }

    #[test]
    fn test_copied(){
        let v1 = Option::Some(&3);
        let v2 = v1.copied();
        println!("v1:{v1:?} v2:{v2:?}");
    }

}
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add to waitlist")
        }
    }
}

 use crate::front_of_house::hosting;

fn brother_call(){
    println!("brother call");
    hosting::add_to_waitlist();
}

mod customer {
    use crate::{
        brother_call,
        customer,
        hosting //这里应用到
    };

    pub fn eat_at_restaurant() {
        //compile error:
        hosting::add_to_waitlist();
        //crate::front_of_house::hosting::add_to_waitlist();
    }

    #[test]
    fn test_eat(){
        customer::eat_at_restaurant();
        brother_call();
    }
}
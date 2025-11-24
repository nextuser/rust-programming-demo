use std::io;
use rand;
use rand::{Rng};

fn main(){
    const MAX:u32 = 50;
    let mut buf = String::new();
    //这个使用新的随机数API
    let target = rand::rng().random_range(0..=MAX);
    ///println!("Target: {target}");  //调试的时候先打印出结果

    'outer: loop {
        println!("Guess the number 0..{MAX}:");
        io::stdin().read_line(&mut buf).expect("Failed to read line");
        let n = buf.trim().parse::<u32>().expect("need number");
        match n {
            //这里不能直接用 target => , 因为会创建一个新变量target
            _ if n == target  => {
                println!("You win!");
                break 'outer;
            }
            _ if n < target => println!("Too small!"),
            _ => println!("Too big!"),

        };
        //下次读取之前，需要清空缓冲区
        buf.clear();

    }

}
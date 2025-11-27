use std::fs::File;
use std::io::{self, ErrorKind,Read};

fn read_username_from_file() ->Result<String,io::Error>{
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text:&str) -> Option<char>{
    text.lines().next()?.chars().last()
}

fn positive(v : i32) -> Option<u32>{
    if v < 0 {
        None
    } else{
        Some(v as u32)
    }
}
fn test_pos(v : i32) ->Option<u32>{
    let x = positive(v)?;
    println!(" Pos succ {x}");
    Some(x)
}



fn main() //-> Result<(),Box<dyn std::error::Error>>
{
    let name : &str = "f.txt";
    _ = File::open(name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(name).unwrap_or_else(|error|{
                panic!("fail to create file {name}, error : {error:?}");
            })
        } else{
            panic!("open file error:{error:?}");
        }
    });
    println!(" open succ {name}");

    let ret = read_username_from_file();
    match ret {
        Ok(s) => {
            println!("read out: {s}");
            let c = last_char_of_first_line(&s);
            println!("last char:{c:?}");
        },
        Err(e) => println!("read file error :{e}")
    }
    let ret = test_pos(3);
    println!("test pos 3 result : {ret:?} ");
    let ret = test_pos(-3);
    println!("test pos -3 result : {ret:?} ");
   //read()
}
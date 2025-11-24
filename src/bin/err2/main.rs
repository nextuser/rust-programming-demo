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

fn read()->Result<(),Box<dyn std::error::Error>>{
    let file = File::open("abc.txt")?;
    Ok(())
}
fn main() //-> Result<(),Box<dyn std::error::Error>>
{
    const name : &str = "f.txt";
    let gf = File::open(name).unwrap_or_else(|error| {
        if(error.kind() == ErrorKind::NotFound){
            File::create(name).unwrap_or_else(|error|{
                panic!("fail to create file {name}");
            })
        } else{
            panic!("open file error:{error:?}");
        }
    });
    println!(" open succ {name}");

    let ret = read_username_from_file();
    match(ret) {
        Ok(s) => {
            println!("read out: {s}");
            let c = last_char_of_first_line(&s);
            println!("last char:{c:?}");
        },
        Err(e) => println!("read file error :{e}")
    }

   //read()
}
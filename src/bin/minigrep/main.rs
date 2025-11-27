use std::env;
use std::ffi::OsString;
use std::fs;
struct Param{
    query : String,
    file : String
}
fn parse_config(args:&Vec<OsString>) ->Param{
    if args.len() < 3 {
        panic!("not enough arguments, minigrep query  filename");
    }
    Param{
        query : args.get(1).expect("need query string arg").clone().into_string().unwrap(),
        file : args.get(2).expect("need file arg").clone().into_string().unwrap()
    }
}
fn main(){
    let args : Vec<OsString> = env::args_os().collect();
    //dbg!(&args);
    let param = parse_config(&args);
    println!("try to read file {:?}",&param.file);
    let contents = fs::read_to_string(&param.file).expect("read file failed");
    println!("search result of `{}` from {}:",&param.query,&param.file);
    for x in contents.lines() {
        if x.contains(&param.query) {
            println!("{x}");
        }
    }
}
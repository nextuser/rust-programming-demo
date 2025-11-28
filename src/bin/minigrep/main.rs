use std::env;
// use std::ffi::OsString;
// use std::fs;
// mod config;
// use config::Config;
// //use crate::Config;
// struct Param{
//     query : String,
//     file : String
// }
// fn parse_config(args:&Vec<OsString>) ->Param{
//     if args.len() < 3 {
//         panic!("not enough arguments:\n minigrep query  filename");
//     }
//     Param{
//         query : args.get(1).expect("need query string arg: \n").clone().into_string().unwrap(),
//         file : args.get(2).expect("need file arg").clone().into_string().unwrap()
//     }
// }
mod config;
use config::Config;

use std::ffi::OsString;
fn main(){
    let args : Vec<OsString> = env::args_os().collect();
    let args :Vec<String> = args.iter().map(|e| e.clone().into_string().expect("invalid into string")).collect();
    //dbg!(&args);
    let config = Config::build(&args);
    println!("after build:");
    if let Err(msg) = config {
        println!("error: {msg}");
    } else  {
        println!("before run");
        if let Err(value) = config.unwrap().run() {
           println!("搜索错误:{}", &value);
        }
    }

    // if let Some(v) = result {
    //     for line in v {
    //         println!("{}",line);
    //     }
    // }
    // if let Err(v) = result {
    //     println!("error : {:?}",v);
    // }

}
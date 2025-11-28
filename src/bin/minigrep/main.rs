use std::env;
//由于代码在bin下面，需要再Cargo.toml定义[lib]
use minigre_lib::config::Config;
//mod config;
use std::ffi::OsString;
//use config::Config;

fn main(){
    let args : Vec<OsString> = env::args_os().collect();
    let args :Vec<String> = args.iter().map(|e| e.clone().into_string().expect("invalid into string")).collect();
    //dbg!(&args);
    let config = Config::build(&args).unwrap_or_else({
        |err| {
            eprintln!("{}",err);
            std::process::exit(1);
        }
    });

    config.run().unwrap_or_else(|err| println!("搜索错误:{}", &err));


    // if let Some(v) = result {
    //     for line in v {
    //         println!("{}",line);
    //     }
    // }
    // if let Err(v) = result {
    //     println!("error : {:?}",v);
    // }

}
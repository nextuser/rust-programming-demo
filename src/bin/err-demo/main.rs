use std::fs::File;
use std::io::ErrorKind;

fn main(){
    const FNAME: &str = "hello.txt";
    let result = File::open(FNAME);
    _ = match result {
        Ok(file ) => file,
        Err(error   ) => match error.kind() {
            ErrorKind::NotFound => match File::create(FNAME) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file:{e:?}")
            },
            _ => panic!("open file error {error:?}")
        }
    };
    println!("open file succs")
    

}
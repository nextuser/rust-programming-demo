use std::fs::File;
use std::io::ErrorKind;

fn main(){
    const fname : &str = "hello.txt";
    let result = File::open(fname);
    let g_file = match result {
        Ok(file ) => file,
        Err(error   ) => match error.kind() {
            ErrorKind::NotFound => match File::create(fname) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file:{e:?}")
            },
            _ => panic!("open file error {error:?}")
        }
    };
    println!("open file succs")
    

}
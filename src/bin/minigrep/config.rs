use std::fs;
use std::error::Error;
use std::path::Path;
pub struct Config{
    query : String,
    file : String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments:\n minigrep query  filename");
        }
        Ok(Config {
            query: args.get(1).unwrap().clone(),
            file: args.get(2).unwrap().clone()
        })
    }

    pub fn run(&self) -> Result<(),Box<dyn Error>>{
        println!("search {} inf file {}",self.query,self.file);
        let path = Path::new(&self.file);

        if ! path.exists(){
            return Err("没有找到目录:{path}".into());
        } else if ! path.is_file(){
            return Err("{} 这个路径不是文件".into());
        }
        println!("begin to read_to_string {}",&self.file);
        let contents = fs::read_to_string(&self.file)?;

        println!("search result of `{}` from {}:",&self.query,&self.file);
        for x in contents.lines() {
            if x.contains(&self.query) {
                println!("{x}");
            }
        }
        Ok(())
    }
}
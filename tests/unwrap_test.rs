use std::fs::File;

fn read() ->Result<(),Box<dyn std::error::Error>>{
    _ = File::open("abc.txt")?;
    Ok(())
}

#[test]
fn test_read(){
    let ret = read();
    assert!(ret.is_err());
}
#[allow(unused_doc_comments)]
fn main(){
    let mut n = 3u8;
    n += 255;

    /**
    1. 调试版本会有panic, 发布版本输出: '2' ,相关命令
    ```shell
     cargo run --example overflow_demo
    ```
    2. 执行release 版本的命令 :
    ```shell
    cargo run --example overflow_demo --release
    ```
    */
    println!("u8: 3 + 255 => {}",n);

}
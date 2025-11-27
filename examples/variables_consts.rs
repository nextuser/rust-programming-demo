//常量能放到global scope, 命名会导致warning，不会导致error
const OUTER_COST :u32 = 32;
fn main(){
    const B :u32 = 2;
    //常量表达式在编译器也可以计算出来，虽然是表达式
    const A :u32 = B * OUTER_COST;

    println!("const A = {A}!");

}
/**
展示对象的Copy ，Dereference 
*/
#[cfg(test)]
mod tests {
    struct MyInt {
        name: [u8; 32],
        value: i32,

    }

    impl Clone for MyInt {
        fn clone(&self) -> Self {
            println!("MyIntCopy clone: value:{}", &self.value);  // Debug print here
            MyInt { name: self.name.clone(), value: self.value }
        }
    }

    trait Dec {
        fn dec(&mut self);
    }

    impl MyInt {
        //定义实例方法，  需要用self
        fn inc(&mut self) {
            self.value += 1;
        }
        //第一定义实例方法，
        fn abs(self) -> i32 {
            if self.value < 0 {
                -self.value
            } else {
                self.value
            }
        }
        //定义关联方法，使用Self表示类型，不是 self，不是实例方法
        fn show(other: &Self) {
            println!("show: value:{}", other.value);
        }
    }

    impl Dec for MyInt {
        fn dec(&mut self) {
            self.value -= 1;
        }
    }

    impl Copy for MyInt {}


    struct Destroyable(i32);

    impl Destroyable {
        fn value(&self) -> i32 {
            self.0
        }

        /**
        * 如果对象没有实现Copy trait，那么这个对象作为参数在函数调用之后，对象不可访问
        */
        fn into(self) -> i32 {
            println!("into: value:{}", self.0);
            self.0
        }
    }

    #[test]
    #[allow(dead_code)]
    fn test_deref() {
        let msg  = r###"
        这个例子演示：
        有些调用是隐含的解引用
        有些调用是显式地解引用
        使用Self函数是关联访问
        使用self参数是实例访问，可能触发self 的copy或move
        使用&self参数是实例访问
        "###;
        println!("{msg}");
        let mut value = MyInt { name: [96u8; 32], value: -34 };
        let c = value.clone();
        
        println!("cloned value{}",c.value);
        
        value.inc();

        println!("after inc {}", value.value);
        let abs_val = MyInt::abs(value); //这里触发了copy 方法， 因为如果没有实现Copy trait， 这个调用会编译错误
        let ref_value = &mut value; //引用
        ref_value.inc();
        let abs_val2 = MyInt::abs(*ref_value); //显式 dereference, copy...
        ref_value.inc();
        let abs_val3 = ref_value.abs(); //隐式 dereference,copy

        println!("{abs_val} {abs_val2} {abs_val3}");

        ref_value.dec();
        MyInt::show(ref_value);
    }

    #[test]
    fn test_destroy() {
        let msg  = r###"
        这个例子演示：
        一个实例方法，使用self 参数，
        如果对象没有实现Copy trait，那么这个参数在函数调用之后，对象不可访问
        "###;
        println!("{msg}");
        let destroyable = Destroyable(34);
        println!("before into: value:{}", destroyable.value());
        let value = destroyable.into();
        println!("after into: value:{}", value);
        //println!("after into: value:{}", destroyable.0); //编译错误,对象已经不可访问
    }

}

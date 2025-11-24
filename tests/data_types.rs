#[cfg(test)]

mod tests {
    #[test]
    fn test_num() {
        let value = b'A';
        println!("hex:{:x}  binary:{:b}  dec:{} char:{}",
                 &value, &value, &value, char::from(value)); //65  = 0x41
        println!("hex:{:#06x}  binary:{:#06b}  oct:{:#06o} {}",
                 &value, &value, &value, &value);
        println!("hex:{:<8x}  binary:{:<8b}  oct:{:#<8o} {}",
                 &value, &value, &value, &value);
        println!("hex:{:>8x}  binary:{:>8b}  oct:{:#>8o} {}",
                 &value, &value, &value, &value);
        assert_eq!(0b0100_0001, b'A');

        let b = value.wrapping_add(255);
        println!("b is {:#06x} {:#06x}", &value, &b);

        let d = value.checked_add(255);
        // checked_add ,溢出返回None
        match d {
            Some(x) => println!("d is {:#06x}", &x),
            None => println!("d is None")
        }
    }

    #[test]
    #[should_panic]
    fn test_overflow() {
        let value = 3u8;
        let c = value + 255; //will panic
        println!("c is {:#06x} {:#06x}", &value, &c)
    }


    fn sign(a: i32) -> i32 {
        if a < 0 {
            -1
        } else {
            1
        }
    }

    fn my_div(left: i32, right: i32) -> (i32, i32) {
        let divisor = left.abs() / right.abs() * sign(right) * sign(left);
        (divisor, (left.abs() % right.abs()) * sign(left))
    }
    fn div_remainser(a: i32, d: i32) {
        //余数符号和被除数一致。
        let remainder = a % d;
        let s = a / d;
        assert_eq!(s * d + remainder, a);
        println!("{} / {} =  {}, remainder {}", a, d, s, remainder)
    }

    fn compare_div(left: i32, right: i32) {
        let diviser = left / right;
        let remainder = left % right;
        let (diviser2, remainder2) = my_div(left, right);
        assert_eq!(diviser, diviser2);
        assert_eq!(remainder, remainder2);
    }
    #[test]
    fn test_div() {
        div_remainser(-7, 5);
        div_remainser(-7, -5);
        div_remainser(7, -5);
        div_remainser(7, 5);
    }

    #[test]
    fn test_compare_div() {
        compare_div(-7, 5);
        compare_div(-7, -5);
        compare_div(7, -5);
        compare_div(7, 5);
    }

    #[test]
    fn test_size() {
        let c1: char = char::from(b'a');
        let c2: char = '中';
        assert_eq!(size_of_val(&c1), 4);
        assert_eq!(size_of_val(&c2), 4);
    }

    #[test]
    fn test_tuple() {
        //tuple的成员可以有不同类型
        let record = ("charles".to_string(), 33, 2000.00);
        //tuple 析构成变量 使用模式匹配将元组值分解到变量中
        let (name, age, income) = record;
        println!("{} is {} years old, earns ${}", name, age, income);
        assert_eq!(age, 33);
        let mut c: (String, i32) = ("charles".to_string(), 44);
        // 访问tuple成员，修改一部分
        c.0 = "charles2".to_string();
        c.1 += 1;
        println!("{} is {} years old", c.0, c.1);
    }

    #[test]
    fn array_vector(){
        // - array 分配在stack ，长度固定
        // - vector 分配在heap，长度可变
        // 数组和vector 每个成员类型一样
    }
    //shadowing
    #[test]
    fn test_shadowing(){
        let x = 5;
        let x = x + 1;
        {
            let x = x * 2;
            assert_eq!(x,12)
        }
        assert_eq!(x,6);
    }
}
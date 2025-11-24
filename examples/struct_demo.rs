#[allow(dead_code)]
#[derive(Debug)]
struct User{
    active: bool,
    sign_account : u64,
    email : String,
    name : String,

}

fn build_user(name :String, email:String) -> User{
    User{
        name, //简明语法 变量名相同省略key
        email,//简明语法,省略key
        active : true,
        sign_account : 1,

    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct FooWithRef<'a>{
    name : &'a str,
    email : &'a str,
    sig_account : u64
}

// 翻譯簡化賦值，key值和變量名相同時，可以省略k值
fn build_fooref<'b>( name : &'b str, email : &'b str,account : u64) ->FooWithRef<'b> {
    FooWithRef{
        name,
        email,
        sig_account:account
    }
}

struct Point{
    x : i32,
    y : i32
}

#[allow(dead_code)]
impl Point{
    //实例方法以self开头
    fn dist_from_origin(&self) -> i32{
      i32::isqrt(self.x*self.x + self.y * self.y)
    }

    fn new(x : i32,y : i32) -> Point{
        Point{
            x,
            y
        }
    }

    fn setX(&mut self,value : i32){
        self.x = value;
    }
    fn setY(&mut self,value : i32){
        self.y = value;
    }
    //这样写关联函数,类似 &self,但是不能当做 point.show() 调用
    fn show(arg : &Self){
        println!("Point show: Point {},{}", arg.x,arg.y);
    }
}

struct Color(i32,i32,i32);
#[allow(deadcode)]
impl Color{
    fn red(&self)->i32{
        self.0
    }
}

fn test_color(){
    let c = Color(1,2,3);
    impl Color{
        fn green(&self) ->i32{
            self.1
        }

        fn msg(&self) ->String {
            format!("r {} g {} b{}", &self.0,&self.1,&self.2)
        }
    }
    println!("green:{}",c.green());

}

fn test_point_borrow_mut(){
    println!("-----------test_point_borrow_mut");
    let mut p = Point{x:1,y:2};
    let x = &mut p.x;
    let y = &mut p.y;//分别借用一部分字段,互相不影响.
    *x = 3;
    * y = 4;
    println!("{} {}", p.x,p.y);
}

struct AlwaysEq;

fn main(){
    let user1 = build_user("ljl".to_string(), String::from("nextuser@163.com"));

    let user2 = User{
        name : "charles".to_string(),
        email : "charles@gmail.com".to_string(),
        .. user1 // update 语法, 剩余两个字段都是impl Copy trait了, 触发copy
    };

    println!("User1:{user1:?}, User2{user2:?}");

    let user3 = User{
        name : "dick".to_string(),
        ..  user1  // 导致了email被move
    };
    println!("user2 {user2:?}");
    //println!("user1 {user1:?}")// 编译错误,email被move了
    println!("user3 {user3:?}");
    //let a1 :AlwaysEq = AlwaysEq;
    //let a2 = AlwaysEq; TODO

    let name4 = "nameStatic";
    let email4 =  "a@b.com".to_string();
    let fooRef = build_fooref(name4,&email4,3);
    println!("fooRef : {fooRef:?}");

    test_point_borrow_mut();
    let mut p = Point::new(3,4);
    println!("dist {}", p.dist_from_origin());
    p.setX(30);
    Point::setY( &mut p,40);
    println!("dist {}",p.dist_from_origin());//不同调用方式访问
    //p.show(); 编译错误
    Point::show(&p);
    test_color();
}
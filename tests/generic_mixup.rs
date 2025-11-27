#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
struct Point<X,Y>{
    x : X,
    y : Y
}

impl<X1,Y1> Point<X1,Y1>
{
    fn mixup<X2,Y2>(self ,other:Point<X2,Y2>) -> Point<X1,Y2>{
        let Point{x,y:_} = self;
        let Point{x:_, y} = other;
        Point{
            x:x,
            y:y
        }
    }
}

impl <X,Y> PartialEq for Point<X,Y>
where X :PartialEq , Y:PartialEq
{
    fn eq(&self,other : &Self) -> bool{
        ( self.x == other.x) && (self.y == other.y)
    }
}

#[test]
fn test_mixup(){
    let p1 = Point{x:3i32,y:4i32};
    let p2 = Point{x:-8f32,y:-9f32};
    let p3 = p1.mixup(p2);
    let expect : Point<i32,f32> = Point{x:p1.x,y:p2.y};
    assert_eq!(p3,expect);
}


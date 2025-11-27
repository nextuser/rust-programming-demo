#[derive(Debug)]
struct Point<T,U>
where T:Copy,U:Copy
{
    x:T,
    y:U
}

#[allow(dead_code)]
impl <T,U> Point<T,U>
where T:Copy,U:Copy
{
    fn x(&self) ->T {
        self.x
    }

    fn y(&self) ->U {
        self.y
    }

}


impl Point<f32,f32> {
    fn distance_from(&self,other: &Point<f32,f32>) -> f32{
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    fn distance_from_origin(&self)->f32{
        self.distance_from(&Point{x:0f32,y:0f32})
    }
}


#[test]
fn test_generic(){
    let pif = Point{x:3,y:4.0};
    let v = pif.x;
    println!("pif.x {v}");

    let pff = Point{x:30f32,y:40f32};

    println!("{pff:?} distance:{}",pff.distance_from_origin())
}
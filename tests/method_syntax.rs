#[cfg(test)]
mod tets{
    #[derive(Debug)]
    struct Rectangle{
        width : u32,
        height : u32,
    }

    impl Rectangle{
        fn area(&self )->u32{
            self.width * self.height
        }

        fn set_width(&mut self,width : u32){
            self.width = width;
        }

        fn max(self, other:&Rectangle) ->Rectangle{
            Rectangle{
                width: self.width.max(other.width),
                height:self.height.max(other.height)
            }
        }
    }
    #[test]
    fn test_ro(){
        let  mut rect = Rectangle{
            width:0,
            height:0
        };
        rect.set_width(3);

        println!("{}", rect.area());
        let other_rect = Rectangle{width:1,height:1};
        let max_rect = rect.max(&other_rect);
        println!("max rect{max_rect:?}");
    }

    #[test]
    fn test_row(){
        let mut rect = Rectangle{
            width:0,
            height:0
        };
        rect.set_width(2);
        let rect_ref = &mut rect;

        rect_ref.set_width(3);
        println!("rectRef {rect_ref:?}");

    }



       #[test]
    fn test_move(){
        let  mut rect = Rectangle{
            width:0,
            height:0
        };
        rect.set_width(3);

        println!("{}", rect.area());
        let other_rect = Rectangle{width:1,height:1};

        let max_rect = rect.max(&other_rect);
       // println!("rect area {}",rect.area());  compiling error
       println!("max_rect : {max_rect:?}");
    }
}

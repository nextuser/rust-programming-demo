
pub trait Summary{
    fn summarize(&self) ->String;
}

pub struct NewsArcticle{
    pub headline :String,
    pub location : String,
    pub author :String,
    pub content : String,
}

impl Summary for NewsArcticle {
    fn summarize(&self) ->String {
        format!("{},by {}, ({})", self.headline,self.author,self.content)
    }
}
pub struct SocialPost{
    pub username :String,
    pub content : String,
    pub reply : bool,
    pub epost : bool,
}

impl Summary for SocialPost{
    fn summarize(&self)->String{
        format!("{} : ({})",self.username, self.content)
    }
}

fn call_summery(value : &impl Summary){
    println!("summary is {}", value.summarize());
}

fn notify<T:Summary> (value :&T){
    println!("ntoify ,summarize if {}", value.summarize())
}
#[test]
fn test_call(){
    let info1 =
        NewsArcticle{headline:"arch head".to_string(),
                    location:"newyork".to_string(),
                    author:"charles".to_string(),
                    content : "some new content".to_string()
        };
    let info2 = SocialPost {
            username: "bob".to_string(),
            content: "one post in net".to_string(),
            reply: false,
            epost: true,
        };


    call_summery(&info1);
    call_summery(&info2);
    notify(&info2);
}

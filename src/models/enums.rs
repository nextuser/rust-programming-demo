// use std::fmt::Display;

#[derive(Debug)]
pub enum YesOrNo{
    Yes,
    No
}


// impl Display for YesOrNo{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }

//如果实现了Display， 就会自动实现ToString，因此不实现Display
impl ToString for YesOrNo{
    fn to_string(&self) -> String{
        match self{
            YesOrNo::Yes => "Yes".to_string(),
            YesOrNo::No => "No".to_string()
        }
    }
}



pub fn show(){
    println!("models::enums::show");
}


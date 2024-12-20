fn fix_incorrect_order() {
    cook_order();
    //super specify the parent module which here is crate
    super::serve_order();
}
mod test_mod;
fn cook_order() {}

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast{
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast{
            toast:String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

pub enum Appetizer {
    Soup,
    Salad,
}
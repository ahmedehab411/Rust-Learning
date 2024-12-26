use std::fmt::{Display,Debug};

/*Trait is a method we can apply for items of type that apply the trait. */
/*Here we are creating 2 new struct types which are NewArticle and Tweet types
to apply Traits for both of them.*/
pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct Test {
    pub user: String,
}
/*Implementing a Summary trait which is public and add a method called summarize to it.
so now if any type implements Summary trait it should implement summarize method. */
pub trait Summary {
    /*applying a default implementation for summarize so now if it's not implemented
    for any type that implement the trait it will get the default implementation*/
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}
/*implementing the trait for NewArticle type. */
impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

/*implementing the trait for Tweet type. */
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {} ",self.username, self.content)
    }
}

impl Summary for Test {

}

/*now we will create a notify function that takes item that implements summary trait */
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
/*Another way to implement it like this so now this generic is limited
to items that implement the summary trait.*/
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/*we can implement a function that takes two arguments of summary trait implemented
types in this way */
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}
/*or simply in this way */
pub fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}  

/*for items that implement more than 2 trait we can use as following: */
pub fn some_function1<T:  Summary + Display>(item:&T) {

}
/*or even we can use it like this for different items types: */
pub fn some_function2<T, U>(t: &T, u: &U) ->i32 
    where T: Display + Clone,
            U: Clone + Drop
{
    5
}

/*now i will implement a function that returns type that implements summarize */
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Ahmed Ehab"),
        content: String::from("My name is Ahmed Ehab"),
        reply: false,
        retweet: false,
    }
}

/*we can implement trait for any type that implements another trait 
here we will implement Summary for any type implements Display*/
impl<T: Display> Summary for T {
    
}
fn main() {
    let article = NewArticle {
        headline: String::from("penguins win the Stanley Cup Championship!"),
        location: String::from("Cairo, Egypt"),
        author: String::from("Ahmed Ehab"),
        content: String::from("The penguins once again are the best hockey team in the NHL."),
    };

    let tweet = Tweet {
        username: String::from("Ahmed Ehab"),
        content: String::from("My name is Ahmed Ehab"),
        reply: false,
        retweet: false,
    };
    /*using the traits for each type. */
    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
    println!("Test summary: {}", Test{user: String::from("Ahmed Ehab")}.summarize());
    println!("----------------------");
    notify(&article);
    println!("----------------------");
    println!("summarize returned: {}",returns_summarizable().summarize());
}

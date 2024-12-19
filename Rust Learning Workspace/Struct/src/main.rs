struct User{
    username: String,
    email:  String,
    sign_in_count:  u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
//we can have one or more implementation blocks
impl Rectangle{
    //this is ma method so it has &self in it's params
    fn area(&self) -> u32 {
    return self.height * self.width;
    }
    fn can_hold(&self,other: &Rectangle) ->bool {
        return self.width>other.width && self.height > other.height;
    }
}
impl Rectangle{
    //this is associated function so it doesn't contain &self
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size,
        }
    }
}
fn main() {
    let mut user1: User = User {
        email: String::from("ahmedehab1232@gmail.com"),
        username: String::from("Ahmed-Ehab"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("Ahmed Ehab");
    let user2 = build_user(String::from("newmail"), String::from("anyName"));
    println!("username: {}",user2.username);

    //another way to define to set different fields and inherits other from user2 as example
    let user3 = User{
        email: String::from("user3mail"),
        username: String::from("user3name"),
        ..user2
    };
    
    let rect = Rectangle {
        width: 5,
        height: 10,
    };
    
    let rect1 = Rectangle {
        width: 4,
        height: 8,
    };
    
    let rect2 = Rectangle {
        width: 6,
        height: 12,
    };
    
    //if we tried to print out our rectangle we will get error
    //println!("rectangle: {}",rect);
    //to print this line we have to derive the debug trait (chapter 10) and add :? or add :#? to make it prettier or implement fmt
    println!("rectangle: {:#?}",rect);

    //print rectangle area using area method
    println!("rectangle area is: {}",rect.area());
    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    let rect3 = Rectangle::square(25);
    println!("square is: {:#?}",rect3);
}

fn build_user (email: String, username: String) ->User {
    User{
        /*can be don in this way too as they have same names
          email,
          username,
          sign_in_count: 0,
          active: false, */
        email: email,
        username: username,
        sign_in_count: 1,
        active: true,
    }
}
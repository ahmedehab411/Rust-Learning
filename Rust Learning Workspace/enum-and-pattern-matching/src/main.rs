//Enums allow us to enumerate a list of variance
enum IpAddrKind {
    V4(String),
    V6(String),
}

//Note we can use enum variance instead of more than 1 struct as follow:
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write (String),
    ChangeColor (i32,i32,i32),
}

//we can also define enum methods
impl Message {
    fn hello() {
        println!("Im here");
    }
}
//Struct that is removed later due to the use of enum variances
/* struct IpAddr {
    kind: IpAddrKind,
    address: String,
} */

/*We also have Option enum 
but we don't need to implement it as it's already implemented
enum Option<T> {
    Some(T),
    None,
}*/
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    /*let localhost = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };*/
    
    //another way is to make enum variance by adding (String) to v4 and v6 
    //it also accepts any data type
    let localhost = IpAddrKind::v4(String::from("127.0.0.1"));

    //using Option enum
    let x:i8 = 5;
    let num: Option<i8> = Some(5);
    let num2:Option<i8> = None;
    let z = x+num.unwrap_or(0);

    let some_value = Some(3);
    match some_value {
        Some(3)=> println!("Three!!"),
        _=>(),
    }
    //the last code can be replace by the following
    //it checks if some_value == Some(3) and do something in bracket
    // its' called if let syntax and it's better when we match to 1 value as match needs to include all possible matches.
    if let Some(3) = some_value {
        println!("Three!!");
    }
}

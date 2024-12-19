use std::{io, process::Output};
fn main() {
    let x: u8 = 12; //0-255
    let y: i8 = 10; //-128 - 127
    /*We can't do arithmetic operations on different data-types variables.
    So arithmetic operations on RUST should be on elements of the same d-type.*/
    //let z = x + y;

    //Or we can simply type-cast to another d-type.
    let x =12.0f32; //here x will be defined as f32 float d-type.
    let y =13.5f32; //here y will be defined as f32 float d-type.
    let z = x + y;
    println!("{}", z);
    
    //We can also define large numbers like this:
    let x = 12_000i64; //this defines x = 12,000 of type i64.
    let y = 12_000_i32; //this defines x = 12,000 of type i32.

    //another way to define variables with casting is like this:
    let x = 127_000 as i64; //this defines x = 127,000 of type i64.
    let y = 127_i32; //this defines x = 127 of type i32.
    let z = x / (y as i64); //here we have casted y to i64 to allow arithmetic operation to be done
    println!("{}", z);

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Expected to read line");
    //converting string into integer or numeric values.
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input+2);
}

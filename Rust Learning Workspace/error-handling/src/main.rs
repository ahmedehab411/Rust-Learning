use std::io;

fn main() {
    //Here is a macro that immediately quit the program and put out an error message.
    /*Note : by using RUST_BACKTRACE=1 as env variable it gives you a back trace
    to how the error happened and generated. like error happened at c where it's called
    by b which it's called by a that is called in the main function. */
    
    //a(); //will result in an error.

    /*for recoverable errors there is an enum called Result enum that is defined by default like this:
        enum Result<T,E> {
            Ok(T),
            Err(E),
        }*/
    //Use case of using Result enum
    use std::fs::File;
    use std::io::ErrorKind;
    /*There is another method we will see later called enclosures which will be more readable */
    let f = File::open("hello.text");
    match f {
        Ok(file)=>file,
        Err(error)=> match error.kind() {
            ErrorKind::NotFound =>match File::create("hello.text") {
                Ok(fc)=> fc,
                Err(er)=> panic!("file not created error: {}",er),
            },
            other_error=>panic!("Error: {:?}",other_error),
        }
    };
    /*unWrap method is the same as the first match statement so we can write it like that
    it returns the file if succeed or panic with error if failed*/
    let f = File::open("hello.text").unwrap();
    println!("File opened successfully.");
    //we can use expect to modify the message appear on any panic
    //let f = File::open("new.text").expect("Failed to open new.text");

    //using the function and print it's output for error propagation
    println!("{:?}",read_username_from_file());
}

fn a() {
    b();
}

fn b() {
    c(24);
}

fn c(x:i32) {
    if x ==24 {
        panic!("Don't pass on 24!");
    }
}

/* ------------------------Error propagation------------------------*/
use std::fs::{self, File};
use std::io::Read;
fn read_username_from_file() -> Result<String,io::Error> {
    let mut f = File::open("hello.text")?;
    
    // return the file name if successful and save to f else return the error
    //we can change this whole code with just ? after the line at the top and the same function is done.
    /* let mut f = match f {
        Ok(file)=>file,
        Err(error)=> return Err(error),
    }; */

    let mut s = String::new();
    //return the string inside the text else return the error
    //this too can be simplified to the next code
    /* match f.read_to_string(&mut s) {
        Ok(_)=> Ok(s),
        Err(error)=> return Err(error),
    } */
   f.read_to_string(&mut s)?;
   Ok(s)
   /*Note: ? is almost similar to unwrap but it returns error don't panic it.
    we also can't use it in the main function as the main function doesn't return
    anything. it can only be used in a function that returns Result or Option types.*/

   //The most simplified version of this code is the following
   /*let mut s = String::new();
    File::open("hello.text")?.read_to_string(&mut s)?
    Ok(s) */

   /*it can be simplified to one line too this can be done as this line
   returns the string result or the error it self*/
   
   // fs::read_to_string("hello.text")
}

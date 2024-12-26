struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    /*Dangling reference it's a reference that points to invalid data like the following */
    let r: &i32;
    {
        let x = 5;
        r = &x;
    } // here x is out of scope so r is a dangling reference
      /*Error here is resulted when borrow checker runs in compile time. */
    //println!("r: {}", r); // will result in an error

    /*Here we will try a more generic example*/
    /*in this case of use lifetimes are equal as they are in the main function */
    let string1 = String::from("Hello ");
    let string2 = String::from("World!");
    let mut result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is: {}", result);

    {
        let string3 = String::from("any");
        result = longest(string1.as_str(), string3.as_str());
    } // here string3 is out of scope so it's life time is ended
      //println!("The longest string is: {}", result); //so it will result on an error as string3 life time is ended

    /*Here is an example of life time annotation in structs */
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i: {}", i.part); // here i is still in scope so it will work but will stop working if i is out of scope or novel is out of scope
}

/*Here it will return a reference to a string slice but s1 here has
different life time than s2 and it cn be called any where in the
program so life times is variable so we use lifetime annotation to
enable the borrow checker to know the life time of an object*/
//Here i have added 'a to the function signature and to the return type as a hint to the compiler
// &i32 is a reference to an i32
// &'a str is a reference to a string slice
// 'a is a lifetime annotation
//  & 'a mut i32 is a mutable reference to an i32
/*What we said here is that s1 , s2 and the return value will have the same life time
it doesn't actually change the lifetime it only makes them related to each others.
so the return value will have life time like that of the lowest lifetime of s1 and s2.*/
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

//Life time rules
//1. Each parameter that is a reference gets its own lifetime parameter.
//2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
//3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
//   the lifetime of self is assigned to all output lifetime parameters.
//it the above rules are not met the compiler will result in an error else it will generate a life time for the output automatically
//here it determines that the return value will have the same life time as the input parameter
//automatically and no error will be generated.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/*note if we have 2 references in the function or struct the
compiler can't automatically specify the lifetime so it will
result in an error on compiling */

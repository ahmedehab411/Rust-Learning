/* extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation; */
/*Collections are set of Data structures that are stored in heap by default
as their size can be changed on run time like : Vectors, String and Hash maps.
all of these like any variable that stored into the heap will be dropped 
automatically when go out of the scope.*/
fn main() {
    //let's create a normal array that has fixed size.
    let a = [1,2,3,4,5];
    //now we will create an empty vector of type i32.
    let mut v:Vec<i32> = Vec::new();
    //pushing elements to the vector.
    v.push(1);
    v.push(2);
    v.push(3);

    {
        //creating a vector and push elements at the same time. using vec! macro.
        let v2 = vec![1,2,3];
    } // v2 will be dropped here
    //accessing elements (first method) will get runtime error when we set wrong index.
    println!("The third element in the vector is: {}",&v[2]);
    //accessing elements the save way (second method)
    match v.get(2) {
        Some(element)=>println!("The element is: {}",element),
        None => println!("There is no element at this index"),
    }
    /*if we tried to push element after referencing to an element in the vector we will get an error
     that's bcs we can't have immutable and mutable references at the same time. as if we tried to
     read the element after pushing the vector might be reallocated in a different place and we are
     now pointing to a garbage value. like the following:*/
     let third_element = &v[2];
     v.push(5);
     //println!("the third element is: {}",third_element); //if we un comment this it will result in an error.

     /*------------- vector iterator------------- */
     for i in &mut v {
        *i +=50; //add 50 to each element
        println!("{}",i); //print after addition
     }
     //if we want to store different types of data in a vector we can do the following using enums
     enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
     }

     let row = vec![
        SpreadSheetCell::Int(15),
        SpreadSheetCell::Float(12.5),
        SpreadSheetCell::Text(String::from("hello from spread sheet")),
     ];

     match &row[2] {
        SpreadSheetCell::Int(i)=>println!("it's integer and value is: {}",i),
        SpreadSheetCell::Float(i)=>println!("it's float and value is: {}",i),
        SpreadSheetCell::Text(i)=>println!("it's text and string is: {}",i),
     }

     /*------------------ Strings -------------------------- */
     //Strings are stored as a collection of utf-8 encoded bytes. in utf-8 chars are encoded in 1 to 4 bytes each.
     //utf-8 is extended version of ASCII code where it had only 127 char but utf-8 has all available chars.
     //Creating a new empty string can be achieved by String::new() method
     let mut s1 = String::new();
     //creating string slice
     let s2 = "Hello";
     //converting string slice into a normal string
     let mut s3 = s2.to_string();
     //another way for conversion
     let s4 = String::from(s2);

     //appending string to a string
     s1.push_str("Hello World");
     //appending char to a string
     s1.push('!');
     println!("s1 after appending string and char is: {}",s1);
     //we can also append strings using + operator
     s3 = s3+" World!";
     println!("s3 after appending string using + operator is: {}",s3);
     //concatenating strings using format macro
     let s5 = format!("1- {} ,2- {}",s1,s3);
     println!("s5: {}",s5);

     //Iterate through string bytes
     for (i,byte) in s1.bytes().enumerate() {
        println!("Byte {} is : {}",i,byte);
     }
     
     //Iterate through string chars
     for (i,byte) in s1.chars().enumerate() {
        println!("Char {} is : {}",i,byte);
     }
     //to Iterate through grapheme clusters you will need a crate
     //grapheme cluster is the char with it's shaping chars like َ,ً,ٌ,ِ ....etc for languages that use them.
     /*for g in "أهلاً وسهلاً".graphemes(true){
        println!("{}",g);
     }*/

    /* ---------------- Hashmap ----------------- */
    use std::collections::HashMap;
    let blue = String::from("Blue");
    let red = String::from("Red");

    /* Creating hashmap */
    let mut scores = HashMap::new();
    /*The next line moves the ownership of blue and red to 
    the hashmap so it's not accessible after these lines. */
    scores.insert(blue, 5);
    scores.insert(red, 2);

    //println!("{}",blue) //not accessible.
    /*getting a value using a key from a hashmap */
    let team_name = String::from("Blue");
    let mut score = scores.get(&team_name);
    println!("Team {} has a score of: {:?}",team_name,score);

    /* iterating through the hashmap*/
    for (k,v) in &scores {
        println!("{}: {}",k,v);
    }

    //When we re-enter the value for the existing key it will replace it's value by the new one.
    scores.insert(String::from("Blue"), 10);
    score = scores.get(&team_name);
    println!("Team {} has a score of: {:?}",team_name,score);

    /*another method to avoid over-writing we can use entry - or_insert 
    pair it only inserts if the key doesn't exist in the hashmap.*/
    scores.entry(String::from("Red")).or_insert(15);
    let team_name = String::from("Red");
    score = scores.get(&team_name);
    println!("Team {} has a score of: {:?}",team_name,score);

    
    scores.entry(String::from("Yellow")).or_insert(15);
    let team_name = String::from("Yellow");
    score = scores.get(&team_name);
    println!("Team {} has a score of: {:?}",team_name,score);

    /*making a program to  count no of occurrence of word in the text */
    let text = "Hello world Wonderful world";

    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count +=1;
    }

    println!("{:#?}",word_count);

}

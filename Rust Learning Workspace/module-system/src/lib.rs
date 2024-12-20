/*once lib.rs is found in the src directory so rust will automatically create
 *a library crate with the same name of the pkg and lib.rs will be the crate root.
 *to create library pkg by default use "cargo new --lib pkg_name" this will create
 *pkg with lib.rs created by default and no binary modules like main.rs
 ***********************************************************************************/
 /*Note: module can contain other modules inside it. it also can contains functions,
 structs, enums, traits, constants and so on. and by default child module and any
 thing inside it is private.*/
/* --------------Restaurant Library module ------------------ */
//creating front of the restaurant module
mod front_of_restaurant {
    //it will has hosting module the is related to all hosting duties (functions)
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    //it also will has serving module the is related to all serving duties (functions)
    mod serving {
        fn take_order() {}
        
        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}
/*Here I will define back of restaurant module into new file for good big project's structure.
so we moved the definition into another rs file called back_of_restaurant.rs and the add semi-colon
after the mod back_of_restaurant definition syntax. we can also move sub modules by creating
a directory named back_of_restaurant and make a .rs file of the sub mod in the same way. */
mod back_of_restaurant;


//now we bring the hosting module into scope so now we don't need it's relative or abs path.
//functions too can be done in the same way.
use crate::front_of_restaurant::hosting;

/*if we bring two modules of same name then they will conflict so we either need to 
use relative or abs path or we can rename modules when bring it into scope as follow*/
use std::fmt::Result;
use std::io::Result as IoResult;

//we can use pub keyword with use to we bring the module into scope and external code can access module as well.
pub use crate::front_of_restaurant::hosting;

//use can be used to bring external modules too into scope like rand.
use rand::Rng;

//if we want to bring more than 1 module from the same crate we can use it like this.
use rand::{Rng,CryptoRng};

/*let's say we need to include std::io and std::io::Write so make it like this*/
use std::io::{self,Write};

//to include all public items in io module we can use this
use std::io::*;

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_restaurant::hosting::add_to_waitlist();

    //relative path
    front_of_restaurant::hosting::add_to_waitlist();

    let mut meal = back_of_restaurant::Breakfast::summer("Rye");

    //also we set the function and struct as public but the struct members are still private
    //so we need to mark them as public fields if needed.
    meal.toast = String::from("Wheat");
    
    //in enums to it's private by default. but it's members are public by default.
    let order1 = back_of_restaurant::Appetizer::Soup;
    let order2 = back_of_restaurant::Appetizer::Salad;

    //here we used the module without any path as we used use keyword to bring it into scope.
    hosting::add_to_waitlist();

    //using Rng from rand module
    let secret_number = rand::thread_rng().gen_range(100);
}
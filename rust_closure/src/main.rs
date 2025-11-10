// closures are anonymous functions which are stored in variables or pass as arguments to other functions

use core::prelude::v1;

struct User {
    name: String,
    age: u8,
    salary: u32,

}

// fn validate_user (name: &str) -> bool {
//     name.len() != 0
// }

    fn is_valid_user <V1, V2> (name: &str, age: u8, simple_validator:V1, advance_validator: V2) -> bool 
    
    where
    V1: Fn(&str) -> bool,
    V2: Fn(u8) -> bool,
     {
        simple_validator(name) && advance_validator(age)
     }
fn main() {

    let person_1 = User{
        name: String::from("Some Guy"),
        age: 35,
        salary: 50_000,
    };
    let validate_user_simple = |name: &str|name.len() != 0;
    let validate_user_advance = |age: u8|age >= 30;


    println!("User validity {} ", validate_user_simple(&person_1.name));
    println!("User advance Validity {} ", is_valid_user(&person_1.name, person_1.age, validate_user_simple, validate_user_advance))
}

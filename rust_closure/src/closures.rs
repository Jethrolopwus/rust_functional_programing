

struct Person {
    first_name: String,
    last_name: String
}



pub fn rust_closures() {
    let add = |x, y| {
        println!("x : {}, Y: {} ", x, y);
        x + y
    };
   let result =  add(4, 10);   

   //adding another closure

   let print_result = |x| println!("The result is {} ", (result +  x));
   print_result(20); 

   //mutating the value from a parent scope by borrowing it into the closure 

   let mut p1 =Person{first_name: String::from ("Jethro"), last_name: String::from("Brad")};
   let mut change_name = |new_last_name: &str, new_first_name: &str| {
       p1.last_name = String::from(new_last_name);
       p1.first_name = String::from(new_first_name);
   };
   change_name("John", "Lopwus");


   println!("The Last nae is: {} ", p1.last_name);
   println!("The first name is: {}", p1.first_name);
}
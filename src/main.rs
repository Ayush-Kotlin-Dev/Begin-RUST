mod conditions;
mod loops;
mod strings;
mod owner;
mod enums;
mod structs;
mod borrowNref;
mod patternMatching ;
mod errorH;
mod option_enum;
mod inputOutput;
mod tictactoe;
use rand::Rng;
use chrono::{Local, Utc};



fn main() {
    println!("Hello, world!");

    let x: i32 = -54;
    let y: i32 = 45;
    let z: i32 = x + y;

    println!("The value of z is: {}", z);

    let x = x + 1; // shadowing the previous value of x
    let x = x * 2;
    println!("The shadowed value of x is: {}", x);

    

    // conditions::conditions();
    // loops::loops();
    // strings::strings();
    // owner::ownership();
    // enums::enums();
    // structs::structs();
    // borrowNref::borrow();
    // patternMatching::Patternmatching();
    // errorH::error_handling();
    // option_enum::option_enum();
    inputOutput::inputOutput();
    tictactoe::tictactoe();


    // let mut rng = rand::thread_rng();
    // let random_number = rng.gen_range(1..101);



    // // Crates and modules 
    // println!("Random number: {}", random_number);

    // let now = Utc::now();
    // println!("Current time is: {}", now);

    // let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
    // println!("Formatted time: {}", formatted);
    // //get the local time
    // let local = Local::now();
    // println!("Local time: {}", local);

}

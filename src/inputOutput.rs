use std::io;

pub fn inputOutput(){
    println!("Enter a number");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    println!("You entered {}", input);
 
    
}
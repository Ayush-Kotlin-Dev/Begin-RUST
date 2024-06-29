struct User{
    name : String,
    age : u32,
    active : bool,
}
 

 pub fn structs(){
    let user1 = User{
        name : String::from("Ayush Rai"),
        age : 25,
        active : true,
    };
    println!("User1 name is: {}", user1.name);
    println!("User1 age is: {}", user1.age);
    println!("User1 active is: {}", user1.active);

    
    let user2 = User{
        name : String::from("Ayush Rai"),
        ..user1
    };
    println!("User2 name is: {}", user2.name);

    //tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Black color is: {}, {}, {}", black.0, black.1, black.2);
    
        
}
pub fn ownership() {
    let s1 = String::from("hello-World");
    let s2 = s1;  // s1 is moved to s2

    //only have a single owner at a timev(String & Vectors)
    // This line would cause a compile-time error because s1 is no longer valid
    // println!("{}", s1);
    
    println!("{}", s2);  // s2 owns the String now :-)

    let s3 = s2.clone();  // s2 is cloned to s3
    println!("{}", s2);   // s2 is still valid
    println!("{}", s3);   // s3 is a clone of s2

    let x = 5;
    let y = x;  // Primitive data types like integers are copied :)
    println!("x: {}, y: {}", x, y);
}

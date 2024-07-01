pub fn borrow(){
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    //Dabgling reference
    let reference_to_nothing = dangle();

    //slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);


    //Lifetime annotations
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // } // `x` goes out of scope here, making `r` a dangling reference
    // println!("r: {}", r);

    // To fix this, we can use lifetimes:
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn dangle() -> String {
    let s = String::from("hello");
    // &s  Error: `s` will be dropped here, and its reference will be invalid.
    s
}
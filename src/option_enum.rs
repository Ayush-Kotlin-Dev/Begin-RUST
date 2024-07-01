
fn find_first_a(s:String) -> Option<i32>{
    for (index, character) in s.chars().enumerate(){
        if character == 'a'{
            return Some(index as i32);
        }
    }
    None
}

//ex - 2 
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}


pub fn option_enum(){
    let my_string = String::from("hello ayush's world");
    let result = find_first_a(my_string);
    match result{
        Some(index) => println!("Found the first a at index: {}", index),
        None => println!("No a's found in the string")
    }

    let result = divide(8.0, 2.5);
    match result {
        Some(value) => println!("Result of division: {}", value),
        None => println!("Division by zero is undefined"),
    }

    //ex - 3 unWrap_or  extracts the value inside Some and panics if the value is None
    let result = divide(8.0, 1.5).unwrap();
    println!("Result of division: {}", result);

}
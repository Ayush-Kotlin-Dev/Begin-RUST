use std::fs;

//implementing the Display trait for Point
struct Point<T>{
    x: T,
    y: T
}

pub fn error_handling(){
    let integer_point = Point{x: 5, y: 10};
    let float_point = Point{x: 1.0, y: 4.0};
    let float_point2 = Point{x: "1.0", y: "4"};
    println!("integer_point: x = {}, y = {}", integer_point.x, integer_point.y);
    println!("float_point: x = {}, y = {}", float_point.x, float_point.y);
    println!("float_point2: x = {}, y = {}", float_point2.x, float_point2.y);
    
    //their is an fn that can error out/stop the thread
    let res = fs::read_to_string("hello.txt");
    match res{
        Ok(file_content) => {
            println!("File content: {}", file_content);
        },
        Err(error) => {
            println!("Error reading file: {:?}", error);
        }
    }
    print!("End of error_handling() function\n");

}
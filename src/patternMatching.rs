// Define enum
enum Shape{
    Circle(f64), // Variant Circle with a single f64 value(radius)
    Square (f64),
    Rectangle (f64, f64),
} 

pub fn Patternmatching(){
    let circle = Shape::Circle(2.0);
    let square = Shape::Square(2.0);
    let rectangle = Shape::Rectangle(2.0, 3.0);

    println!("Area of circle is {}", calculate_area(circle));
    println!("Area of square is {}", calculate_area(square));
    println!("Area of rectangle is {}", calculate_area(rectangle));




}

fn calculate_area(shape : Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(length, breadth) => length * breadth,
    }
}
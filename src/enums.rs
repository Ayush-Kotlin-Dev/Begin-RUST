enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64)
}

fn calculate_area(shape:Shape)-> f64{
    
    return 0.00;
    
}

pub fn enums() {
    //
    let light = TrafficLight::Red;
    
    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Caution!"),
        TrafficLight::Green => println!("Go!"),
    }
    //
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    match msg2 {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }

    //
    let circle : Shape = Shape::Circle(5.0);
    let square : Shape = Shape::Square(4.0);
    let rectangle : Shape = Shape::Rectangle(3.0,6.0);

    calculate_area(circle);


}

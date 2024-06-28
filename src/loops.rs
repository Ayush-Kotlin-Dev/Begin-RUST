pub fn loops(){
    //  loops
let mut count = 0;
loop {
    count += 1;
    println!("Loop count: {}", count);

    if count == 9 {
        break;
    }
}

count = 0;
while count < 3 {
    count += 1;
    println!("While loop count: {}", count);
}

for number in 1..4 {
    println!("For loop number: {}", number);
}

let array = [1, 2, 3, 4, 5];
for i in array {
    println!("Array element: {}", i);
}
}
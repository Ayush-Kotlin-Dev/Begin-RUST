pub fn strings(){
    let greeting = String::from("Ayush");

    println!("Hello, {}", greeting);

    let char1 = greeting.chars().nth(2);
    println!("Character at index 2: {:?}", char1);

    let sentence =String::from( "My_name is Ayush");
    let word = first_word(sentence);
    //
    print!("First word: {}", word);


    
}

fn first_word(sentence:String) -> String{
    let mut ans = String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }
    return ans;


}


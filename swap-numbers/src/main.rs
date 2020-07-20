use std::io;

fn main() {
    //Write a program to swap two variables
    let mut number1: u8 = user_input().trim().parse().expect("error");
    let mut number2: u8 = user_input().trim().parse().expect("error");
    
    println!("Before swapping: number1: {}, number2: {}", number1, number2);

    //Method 1: Bu using a temorary variable
    let mut temp = 0;
    temp = number1;
    number1 = number2;
    number2 = temp; 
    println!("After swapping: number1: {}, number2: {}", number1, number2);

    //Method 2: By using mathematical operations
    number1 = number1 + number2;
    number2 = number1 - number2;
    number1 = number1 - number2;
    println!("After swapping: number1: {}, number2: {}", number1, number2);
}

fn user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    input
}

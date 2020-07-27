use std::io;

fn main() {
    //Write a program to calculate the sum of two integers and return true if the sum is equal to a third integer
    let number1: u8 = user_input(String::from("Input the first number: ")).trim().parse().expect("error");
    let number2: u8 = user_input("Input the second number: ".to_string()).trim().parse().expect("error");
    let number3: u8 = user_input(String::from("Input the third number: ")).trim().parse().expect("error"); 

    if number1 + number2 == number3 {
        println!("The result is: {}", true);
    } else {
        println!("The result is: {}", false);
    }
}

fn user_input(string: String) -> String {
    println!("{}", string);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    input
}

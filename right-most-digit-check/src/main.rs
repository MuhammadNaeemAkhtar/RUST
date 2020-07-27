use std::io;

fn main() {
    //Write a program that accepts three integers from the user and return true if two or more of them (integers ) have the same rightmost digit. The integers are non-negative
    let number1: u8 = user_input(String::from("Input the first number: ")).trim().parse().expect("error");
    let number2: u8 = user_input("Input the second number: ".to_string()).trim().parse().expect("error");
    let number3: u8 = user_input(String::from("Input the third number: ")).trim().parse().expect("error"); 

    if number1 % 10 == number2 % 10 || number2 % 10 == number3 % 10 || number1 % 10 == number3 % 10{
        println!("The result is: true");
    } else {
        println!("The result is: false")
    }
}

fn user_input(string: String) -> String {
    println!("{}", string);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    input
}

use std::io;

fn main() {
    //1.Write a program that takes a number as input and prints its multiplication 
    let number: u8 = user_input().trim().parse().expect("error");

    for i in 1..=10 {
        println!("{} X {} = {}", number, i, number*i);
    }   
}

fn user_input() -> String {
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    input
}

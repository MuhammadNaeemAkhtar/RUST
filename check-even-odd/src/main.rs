use std::io;

fn main() {
    //Write a program to accept a number and check the number is even or not. Prints 1 if the number is even or 0 if the number is odd.
    let number: u32 = user_input().trim().parse().expect("error");
    
    //Check Even
    if number % 2 == 0 {
        println!("1");
    }

    //Check Odd
    if number % 2 != 0 {
        println!("{}", 0);
    }
}

fn user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    input
}

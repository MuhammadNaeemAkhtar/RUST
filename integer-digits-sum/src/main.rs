use std::io;
fn main() {
    // Write a program and compute the sum of the digits of an integer. 
    let mut integer = 25; 
    let mut sum = 0;
    while integer > 0 {         //25 > 0 true               2  > 0  true            0 > 0
        sum = sum + integer%10; //0 + 25%10 = 0 + 5 = 5     5 + 2%10 = 5 + 2 = 7
        integer = integer/10;   //25 / 10 = 2               2 / 10 = 0
    }
    println!("{}", sum);   
}

fn user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    input
}

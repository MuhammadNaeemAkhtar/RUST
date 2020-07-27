fn main() {
    //Write a program to print numbers between 1 to 100 which are divisible by 3, 5 and by both

    //numbers divisible by 3
    println!("Divided by 3:");
    for number in 1..=100 {
        if number % 3 == 0 {
            print!("{}", number);
        }
    }
    println!();

    //numbers Divided by 5
    println!("Divided by 5:");
    for number in 1..=100 {
        if number % 5 == 0 {
            if number == 100 {
                print!("{}", number);
            } else {
                print!("{}, ", number);
            }
        }
    }
    println!();

    //numbers divided by 3 and 5
    println!("Divided by 3 and 5:");
    for number in 1..=100 {
        if number % 3 == 0 && number % 5 == 0 {
            if number == 100 {
                print!("{}", number);
            } else {
                print!("{}, ", number);
            }
        }
    }
    println!();
}

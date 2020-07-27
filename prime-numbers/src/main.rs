fn main() {
    //Write a program to print the odd numbers from 1 to 99. Prints one number per line. 
    //Odd Numbers  not divisible by 2 -> remainder not equal to zero
    //Even Numbers divisible by 2 -> remainder equal zero
    
    for number in 1..=99 {
        //Method 1
        /* if number % 2 != 0 {
            println!("{}", number);   
        } */

        //Method 2
        if number % 2 == 0 { 1
            continue;
        }
        println!("{}", number);
    }
}

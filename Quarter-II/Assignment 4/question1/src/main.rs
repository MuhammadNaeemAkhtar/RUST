fn main() {
    let mut score = 2;
    let mut add_2 = || {
        score +=  2;
        println!("{}", score);
    };
    for i in 0..10 {
        add_2();
    }
}

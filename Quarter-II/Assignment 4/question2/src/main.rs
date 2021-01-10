use std::thread;
use std::time::Duration;

fn main() {
    let mut score = 2;
    let mut goal = 2;
    let mut add_2 = || {
        score +=  2;
        println!("{}", score);
    };
    for _i in 0..10 {
        add_2();
    }
    let multiply_goal = thread::spawn( move || {
        for _i in 0..15 {
            goal *= 2;
            println!("{}", goal);
            thread::sleep(Duration::from_millis(20));
        }
    });
    multiply_goal.join().unwrap();
}

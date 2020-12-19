#[derive(Debug)]
struct Car <T,U,Y> {
    make: T,
    length: U,
    width: Y
}

fn main() {
    let car = Car {
        make: String::from("Honda"),
        length: 15,
        width: 14.6
    };
    println!("{:#?}", car);
}

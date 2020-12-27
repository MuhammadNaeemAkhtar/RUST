//4. Go through the solution of the largest function given at the end of 10.2 in the book and
//rewrite the solution but this time returning the smallest item instead largest.
fn smallest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut smallest = list[0];

    for &item in list {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = smallest(&number_list);
    println!("The smallest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = smallest(&char_list);
    println!("The smallest char is {}", result);
}
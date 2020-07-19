#[derive(Debug)]

struct Student {
    name: String,
    grade: char,
    age: u8,
    percentage: f32,
}

impl Student {
    fn instance(name: String, grade: char, age: u8, percentage: f32) -> Student {
        Student {
            name,
            grade,
            age,
            percentage,
        }
    }

    fn percentage(&self) {
        println!("{}",self.percentage);
    }
}

fn main() {
    let naeem = Student::instance(String::from("Muhammad Naeem Akhtar"),'A',26, 83.33);
    println!("{:#?}", naeem);
    naeem.percentage();
}

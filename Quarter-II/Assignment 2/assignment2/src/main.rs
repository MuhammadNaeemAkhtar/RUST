//a. Define a struct IOT_student with attributes (name, age, education).
#[derive(Debug)]
struct IOT_student {
    name: String,
    age: i32,
    education: String
}

//b. Define another struct IOT_instructor (name, age).
#[derive(Debug)]
struct IOT_instructor {
    name: String,
    age: i32
}

//c. Define a trait Questions with method ask_Questions with a default
//implementation which prints (“Zoom session will be LIVE, Zoom recording will
//    not be available. Quarter 2 studio recorded videos are available on Portal.”).
trait Questions {
    fn ask_questions(&self, name: String) {
        println!("Zoom session will be LIVE, Zoom recording will not be available.
        Quarter 2 studio recorded videos are available on Portal.")
    }
}

//d. Impl trait Questions for IOT_instructor which overrides the default implementation
//of method ask_question, takes student name as a parameter and prints on
//screen (“{} In case of any issue email to education@piaic.org”).
impl Questions for IOT_instructor {
    fn ask_questions(&self, name: String) {
        println!("{:?} In case of any issue email to education@piaic.org", name);
    }
}

//e. Create instances of both the structs and call Method ask_question.

fn main() {
    let naeem = IOT_student {
        name: String::from("Naeem"),
        age: 28,
        education: String::from("Master")
    };
    let sir_imran = IOT_instructor {
        name: String::from("Sir Imran"),
        age: 35
    };
    sir_imran.ask_questions(naeem.name);
}

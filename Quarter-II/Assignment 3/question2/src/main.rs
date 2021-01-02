#[derive(Debug)]
struct Students<'a> {
    name: &'a str,
    age: &'a str,
    education: &'a str,
    timing: &'a str
}

impl <'a> Students <'a> {
    fn get_name(&self) -> &'a str {
        self.name
    } 
    fn get_timing(&self) -> &'a str {
        self.timing
    }
    fn get_edu(&self) -> &'a str {
        self.education
    }
}

fn main() {
    let naeem = Students {
        name: "Naeem",
        age: "28",
        education: "Master",
        timing: "09-13"
    };
    let salman = Students {
        name: "Salman",
        age: "20",
        education: "Bachelor",
        timing: "13-18"
    };
    let haidar = Students {
        name: "Haidar",
        age: "24",
        education: "Bachelor",
        timing: "13-18"
    };
    println!("{:#?}", naeem.get_name());
    println!("{:#?}", naeem.get_timing());
    println!("{:#?}", naeem.get_edu());
    println!("{:#?}", salman.get_name());
    println!("{:#?}", salman.get_timing());
    println!("{:#?}", salman.get_edu());
    println!("{:#?}", haidar.get_name());
    println!("{:#?}", haidar.get_timing());
    println!("{:#?}", haidar.get_edu());
}

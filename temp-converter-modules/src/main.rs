mod a {
    pub fn function1(x: f32) -> f32 {
        super::b::function2(x*9 as f32)
    }
}

mod b {
    pub fn function2(x: f32) -> f32 {
        super::c::function3(x/5 as f32)
    }
}

mod c {
    pub fn function3(x: f32) -> f32 {
        x + 32 as f32
    }
}

mod d {
    pub fn function4(x: f32) -> f32 {
        super::e::function5(x - 32 as f32)
    }
}

mod e {
    pub fn function5(x: f32) -> f32 {
        super::f::function6(x * 5 as f32)
    }
}

mod f {
    pub fn function6(x: f32) -> f32 {
        x / 9 as f32
    }
}

fn main() {
    let temp_in_f = a::function1(30.0);
    println!("{}", temp_in_f);
    
    println!("{}", d::function4(temp_in_f));
}

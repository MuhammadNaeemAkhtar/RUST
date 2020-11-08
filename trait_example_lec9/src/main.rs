#[derive(Debug)]
pub struct NewsArticle {
    pub author: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more")
    }
}

impl Summary for Tweet { } //will give an error

fn main() {
    let tweet_1 = Tweet {
        username: String::from("John"),
        content: String::from("honesty is the best policy"),
    };

    println!("{}", tweet_1.summarize());
}

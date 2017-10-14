use std::env;

fn main() {
    let username = env::var("GIT_HTTPSABLE_USERNAME").unwrap();
    let password = env::var("GIT_HTTPSABLE_PASSWORD").unwrap();
    println!("Hello, world!");
}

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let key = env::var("API-KEY").expect("API KEY NOT FOUND");

    println!("KEY: {key}");

    println!("Hello, world!");
}

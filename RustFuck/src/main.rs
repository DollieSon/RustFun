use std::{fs::File, io::Read};
fn main() {
    println!("Hello, world!");
    let mut file = File::open("something.txt").unwrap();
    let mut conts = String::new();
    file.read_to_string(&mut conts).unwrap();
    let mut vec = Vec::<char>::new();
    let mut index: usize = 0;
    let mut chizz = ' ';
    for ch in conts.chars() {
        match ch {
            '+' => {
                index += 1;
            }
            '-' => {
                index -= 1;
            }
            '/' => {
                index = 0;
            }
            '`' => {
                if (index >= vec.len()) {
                    vec.push(chizz);
                } else {
                    vec.remove(index);
                    vec.insert(index - 1, chizz);
                }
            }
            '*' => {
                let word: String = vec.iter().collect();
                println!("{}", word);
            }
            _ => {}
        }
        chizz = ch;
    }
}

use std::{
    fs::File,
    io::{self, Read},
};
fn main() {
    println!("Hello, world!, {}", 1 as char);
    let mut file = File::open("something.txt").unwrap();
    let mut conts = String::new();
    file.read_to_string(&mut conts).unwrap();
    read(&conts);
}
/*
BASE BRAINFUCk
< - increase pointer
> - decrease pointer
+ - incerement memory cell
- - decrement memory cell
. - output/print memory cell
, - ask for input in memory cell
[ - goto ']' if cell pointer is zero
] - goto '[' if cell pointer is non-zero
*/
fn read(buffer: &String) {
    let memory_len = 400;
    let line: Vec<char> = buffer.chars().collect();
    let mut memory: Vec<u32> = vec![0; memory_len];
    let mut mem_index: usize = 0;
    let mut buffer_pointer: usize = 0;
    let mut loop_stack: Vec<usize> = Vec::new();
    let mut weird_stuff = Vec::new(); // comment store
    while buffer_pointer < line.len() {
        // while loop to allow backtracking
        let ch = line.get(buffer_pointer).unwrap();
        match ch {
            '>' => {
                mem_index += 1;
                if mem_index > memory_len {
                    panic!("Memory Corrupton")
                }
            }
            '<' => {
                mem_index -= 1;
                // panics by itself hehe
            }
            '+' => match memory.get_mut(mem_index) {
                Some(cell) => {
                    *cell += 1;
                }
                None => {
                    panic!("This Should not happen, bad programmer me")
                }
            },
            '-' => match memory.get_mut(mem_index) {
                Some(cell) => {
                    if *cell == 0 {
                        panic!(" Ah decrement of 0 on command no. {}", buffer_pointer);
                    }
                    *cell -= 1;
                }
                None => {
                    panic!("This Should not happen, bad programmer me")
                }
            },
            '.' => match memory.get(mem_index) {
                Some(cell) => {
                    print!("{}", char::from_u32(*cell).unwrap_or('_'));
                }
                None => {
                    panic!("error printing: on command no. {},", buffer_pointer);
                }
            },
            ',' => match memory.get_mut(mem_index) {
                Some(cell) => {
                    // HAHAHA UNWRAP 2x CC-COMBO
                    let mut buffer = String::new();
                    std::io::stdin().read_line(&mut buffer).unwrap();
                    let ch = buffer.chars().nth(0).unwrap();
                    *cell = ch.try_into().unwrap_or(0);
                }
                None => {}
            },
            _ => {
                weird_stuff.push(ch);
            }
        }
        buffer_pointer += 1;
    }
}

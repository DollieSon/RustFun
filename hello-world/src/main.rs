use std::{collections::HashMap, result, usize};

fn main() {
    println!("Hello, world!");
}
fn binary_slice_to_number(slice: &[u32]) -> u32 {
    // your code here
    let mut result: u32 = 0;
    let mut counter: u32 = 1;
    for item in slice.iter().rev() {
        result += counter * item;
        counter >>= 1;
    }
    result
}
fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    numbers.iter().positon_min()
}
fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    if a1.len() == 0 || a2.len() == 0 {
        return -1;
    };
    //big .. small
    let a1_mesurements = get_max_min(&a1);
    let a2_mesurements = get_max_min(&a2);
}
fn get_max_min(words: &Vec<&str>) -> (usize, usize) {
    let mut min = usize::MAX;
    let mut max: usize = 0;
    for word in words.iter() {
        let len = word.len();
        if len < min {
            min = len;
        }
        if len > max {
            max = len;
        }
    }
    return (max, min);
}
fn dna_to_rna(dna: &str) -> String {
    dna.chars()
        .map(|x| if x == 'T' { 'U' } else { x })
        .collect()
}

fn rps(p1: &str, p2: &str) -> &'static str {
    match (p1, p2) {
        ("Scissors", "Rock") => {
            return "Player 2 won!";
        }
        ("Scissors", "Paper") => {
            return "Player 1 won!";
        }
        ("Rock", "Paper") => {
            return "Player 2 won!";
        }
        ("Rock", "Scissors") => {
            return "Player 1 won!";
        }
        ("Paper", "Scissors") => {
            return "Player 2 won!";
        }
        ("Paper", "Rock") => {
            return "Player 1 won!";
        }
        (_, _) => {
            return "Draw!";
        }
    }
}

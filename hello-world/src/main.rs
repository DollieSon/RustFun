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

fn number(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops
        .iter()
        .fold(0, |acc, (get_on, get_off)| acc + *get_on - *get_off)
}
fn generate_shape(n: i32) -> String {
    let mut res = "".to_string();
    for num in 0..n {
        res.push_str(format!("{end:+>pad$}", end = "\n", pad = num as usize).as_str());
    }
    res
}
fn solve(strings: &[String]) -> Vec<usize> {
    let mut res = Vec::<usize>::new();
    for str in strings {
        let something = str.as_str().chars().enumerate().fold(0, |acc, (ind, ch)| {
            if ((ch.to_ascii_lowercase() as u32) - 97) == (ind as u32) {
                acc + 1
            } else {
                acc
            }
        });
        res.push(something);
    }
    return res;
}
fn min_sum(xs: &[u64]) -> u64 {
    let mut vrec = xs.to_vec();
    let mut running_sum = 0;
    for x in 0..(vrec.len() / 2) {
        running_sum += vrec.get(x).unwrap() * vrec.get(vrec.len() - 1 - x).unwrap();
    }
    running_sum
}
fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    for x in 0..len {
        let mut lower = Vec::<usize>::new();
        for y in 0..len {
            lower.push(x * y);
        }
        res.push(lower);
    }
    res
}
fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    let mut hashmap = HashMap::<String, i32>::new();
    println!("{:?}", list_cat);
    for cat in list_cat.iter() {
        hashmap.insert(cat.to_string(), 0);
    }
    for art in list_art.iter() {
        let mut some = art.to_string();
        if let Some(x) = hashmap.get_mut(&some.chars().nth(0).unwrap().to_string()) {
            let split: i32 = some.split(' ').nth(1).unwrap().parse().unwrap();
            *x += split;
        }
    }
    println!("{:?}", hashmap);
    let mut res = "".to_string();
    let mut keys = hashmap.keys().collect::<Vec<&String>>();
    //keys.sort();
    for key in list_cat.iter() {
        res.push_str(format!(" ({} : {}) -", key, hashmap.get(&key.to_string()).unwrap()).as_str());
    }

    res[1..res.len() - 2].to_string()
}
fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
    let something: u8 = numbers.iter().map(|mini| mini.iter().min().unwrap()).sum();
    for mini in numbers.iter() {
        println!("{}", mini.iter().min().unwrap());
    }
    println!("{:?}", something);
    return something;
}

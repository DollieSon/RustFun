use std::{
    collections::{HashMap, HashSet},
    result, usize,
};

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
fn duplicate_encode(word: &str) -> String {
    let mut res = "".to_string();
    let mut set = HashMap::<char, u32>::new();
    for ch in word.chars() {
        if let Some(x) = set.get_mut(&ch.to_ascii_lowercase()) {
            *x += 1;
        } else {
            set.insert(ch.to_ascii_lowercase(), 0);
        }
    }
    for ch in word.chars() {
        if let Some(x) = set.get(&ch.to_ascii_lowercase()) {
            if x == 1 {
                res += "(";
            } else {
                res += ")";
            }
        }
    }
    return res;
}

fn find_odd(arr: &[i32]) -> i32 {
    let mut hash = HashMap::<i32, i32>::new();
    arr.iter().for_each(|num| {
        if let Some(val) = hash.get_mut(num) {
            *val += 1;
        } else {
            hash.insert(*num, 1);
        }
    });
    for (key, values) in hash.iter() {
        if values % 2 == 1 {
            return *key;
        }
    }
    return -1;
}
fn hello(name: &str) -> String {
    if name == "" {
        return "Hello, World!".to_string();
    }
    let new_name = name
        .chars()
        .enumerate()
        .map(|(ind, ch)| {
            if ind == 0 {
                ch.to_ascii_uppercase()
            } else {
                ch.to_ascii_lowercase()
            }
        })
        .collect::<String>();

    "Hello".to_string() + &new_name
}

fn count(input: &str) -> HashMap<char, i32> {
    let mut res = HashMap::<char, i32>::new();
    input.chars().for_each(|c| match res.get_mut(&c) {
        Some(count) => {
            *count += 1;
        }
        None => {
            res.insert(c.clone(), 1);
        }
    });
    return res;
}
fn count_duplicates(text: &str) -> u32 {
    let mut counter = HashMap::<char, u32>::new();
    let mut count = 0;
    for ch in text.to_ascii_lowercase().chars() {
        match counter.get_mut(&ch) {
            Some(item) => {
                *item += 1;
            }
            None => {
                counter.insert(ch, 1);
            }
        }
    }
    for (ch, cn) in counter.iter() {
        if *cn > 1 {
            count += 1;
        }
    }
    count
}
// fn longest(a1: &str, a2: &str) -> String {
//     let mut new_string: Vec<char> = (a1.to_string() + a2)
//         .chars()
//         .iter
//         .is_sorted_by(|a, b| a <= b)
//         .collect();
// }
fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut a_temp = a.clone();
    let mut b_temp = b.clone();
    a_temp.sort();
    b_temp.sort();
    let mut iter_a = a_temp.iter();
    let mut iter_b = b_temp.iter();
    while let Some(a_num) = iter_a.next() {
        let b_num = iter_b.next().unwrap();
        if *b_num != *a_num
            && ((*b_num) as f64).sqrt() as i64 != *a_num
            && ((*a_num) as f64).sqrt() as i64 != *b_num
        {
            return false;
        }
    }
    print
    return true;
}

pub mod Game;
use Game::moves::*;

fn gimme(input_array: [i32; 3]) -> usize {
    let mut small:Option<usize> = None;
    let mut middle:Option<usize> = None;
    let mut largest:Option<usize> = None;
    for index in (0..3){
        let item = input_array[index];
        match (small,middle,largest) {
            (_,_,None) => largest = Some(index),
            (_,None,Some(x)) => middle = Some(index),
            (_,Some(_),Some(_))=>small = {
                
            }
            _=>{
                panic!("Oppsie")
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

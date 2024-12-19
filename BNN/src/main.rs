use neural_net::*;

pub mod neural_net;

fn main() {
    let x = 0.65;
    println!("{:?}", sigmoid(&x));
}

use std::env;

mod buckshot;
mod error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut round = buckshot::create_round(args[1].parse().unwrap()).expect("Failed to create round");
    while !round.done() {
        println!("{round}");
        round.shoot(true).unwrap();
    }
    println!("{:?}", round);
}

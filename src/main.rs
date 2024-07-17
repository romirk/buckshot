use std::env;

mod buckshot;
mod error;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    let mut round = buckshot::create_round(3).expect("Failed to create round");
    while !round.done() {
        println!("{:?}", round);
        round.shoot(true).unwrap();
    }
    println!("{:?}", round);
}

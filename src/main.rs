use std::{env, io};
use std::io::{BufRead, Write};

mod buckshot;
mod error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut round = buckshot::create_round(args[1].parse().unwrap()).expect("Failed to create round");

    let live = round.live();
    let blanks = round.bullets() - live;
    println!("\n\x1b[31m{}\x1b[0m live. \x1b[33m{}\x1b[0m blank{}. I insert the shells in an unknown order.", live, blanks, if blanks == 1 { "" } else { "s" });

    let mut lines = io::stdin().lock().lines();

    let mut careful = false;
    while !round.done() {
        println!("{round}");
        if round.players_turn() {
            print!("[d/y] ");
            io::stdout().flush().unwrap();
            let line = lines.next().expect("couldn't read stdin").unwrap();
            match line.trim() {
                "d" => round.shoot(false).unwrap(),
                "y" => round.shoot(true).unwrap(),
                _ => continue
            }
        } else {
            round.shoot(false).unwrap();
        }
        if !careful && round.lives()[0] == 1 {
            println!("\x1b[90mCareful now...\x1b[0m");
            careful = true;
        }
    }
    println!("\x1b[36m{round}\x1b[0m");
}

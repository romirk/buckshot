use std::io;
use std::io::{BufRead, Write};
use std::thread::sleep;
use std::time::Duration;

use crate::typewriter::typewrite;

mod buckshot;
mod error;
mod typewriter;

fn main() {
    let mut round = buckshot::create_round();

    let live = round.live();
    let blanks = round.bullets() - live;
    let magazine = round.debug_magazine();
    typewrite(format!("\n\x1b[31m{}\x1b[0m live.", live));
    sleep(Duration::from_millis(500));
    typewrite(format!(" \x1b[33m{}\x1b[0m blank{}.", blanks, if blanks == 1 { "" } else { "s" }));
    sleep(Duration::from_millis(500));
    typewrite("\nI insert the shells in an unknown order.\n\n".to_string());
    sleep(Duration::from_millis(700));
    let mut lines = io::stdin().lock().lines();

    let mut careful = round.lives()[0] == 1;

    while !round.done() {
        println!("{round}");
        let hit = if round.players_turn() {
            print!("\x1b[90m[d/y]\x1b[0m ");
            io::stdout().flush().unwrap();
            let line = lines.next().expect("couldn't read stdin").unwrap();
            match line.trim() {
                "d" => round.shoot(false).unwrap(),
                "y" => round.shoot(true).unwrap(),
                _ => continue
            }
        } else {
            round.shoot(false).unwrap()
        };
        if hit {
            typewrite("\x1b[31mHIT \x1b[0m ".to_string());
        } else {
            typewrite("\x1b[32mMISS\x1b[0m ".to_string());
        }
        if !careful && round.lives()[0] == 1 {
            println!("\n\x1b[90mCareful now...\x1b[0m");
            careful = true;
        }
    }
    println!();
    let lives = round.lives();
    if lives[0] == 0 {
        typewrite("\n\x1b[31mI WIN\x1b[0m ".to_string());
    } else if lives[1] == 0 {
        typewrite("\n\x1b[32mYOU WIN\x1b[0m ".to_string());
    }
    println!("\n\x1b[36m{magazine}\x1b[0m {round}");
}

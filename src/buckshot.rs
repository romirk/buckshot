use std::{fmt, io};
use std::fmt::Formatter;
use std::io::{BufRead, Write};
use std::thread::sleep;
use std::time::Duration;

use rand::{Rng, thread_rng};

use crate::buckshot::RoundResult::{Continue, DealerWins, PlayerWins};
use crate::typewriter::typewrite;

// #[derive(Eq, PartialEq)]
// enum Item {
//     Adrenaline,
//     Beer,
//     BurnerPhone,
//     Cigarettes,
//     ExpiredMedicine,
//     HandSaw,
//     Handcuffs,
//     Inverter,
//     MagnifyingGlass,
// }


// #[derive(Debug)]
// struct Player {
//     pub lives: u8,
//     // 4 bit item counts in order of Item
//     // items: u64,
// }

#[derive(Debug)]
pub struct Round {
    bullets: u8,
    players_turn: bool,

    // 1 bit per bullet, 1 = live
    magazine: u8,

    dealer_lives: u8,
    player_lives: u8,
}


impl Round {
    fn done(&self) -> bool {
        self.bullets == 0 || self.player_lives == 0 || self.dealer_lives == 0
    }

    fn live(&self) -> u8 {
        self.magazine.count_ones() as u8
    }

    fn shoot(&mut self, suicide: bool) -> Result<bool, &'static str> {
        if self.bullets == 0 {
            return Err("No bullets.");
        }

        if self.player_lives == 0 || self.dealer_lives == 0 {
            return Err("Somebody already died.");
        }


        let live = (self.magazine & 1) != 0;
        self.magazine >>= 1;
        self.bullets -= 1;

        if !live {
            if suicide {
                return Ok(false);
            }
        } else {
            if (suicide && !self.players_turn) || (!suicide && self.players_turn) {
                self.dealer_lives -= 1;
            } else {
                self.player_lives -= 1;
            }
        }
        self.players_turn = !self.players_turn;
        Ok(live)
    }


    fn shuffled_magazine(&self) -> u8 {
        let mut rng = thread_rng();
        let pos = rand::seq::index::sample(&mut rng, self.bullets as usize, self.live() as usize).into_vec();
        let mut magazine: u8 = 0;

        for p in pos {
            magazine |= 1 << p;
        }

        magazine
    }

    pub fn play(&mut self, stage: u8, round: u8) -> RoundResult {
        let live = self.live();
        let blanks = self.bullets - live;
        // let magazine = print_magazine(self.bullets, self.magazine);

        println!();
        match stage {
            0 => {
                typewrite(print_magazine(self.bullets, self.shuffled_magazine()));
                typewrite(format!("\n\x1b[31m{}\x1b[0m live rounds.", live));
                sleep(Duration::from_millis(500));
                typewrite(format!(" \x1b[33m{}\x1b[0m blank{}.", blanks, if blanks == 1 { "" } else { "s" }));
                sleep(Duration::from_millis(500));
                typewrite("\nI insert the shells in an unknown order.\n\n".to_string());
                sleep(Duration::from_millis(700));
            }
            1 => {
                typewrite(print_magazine(self.bullets, self.shuffled_magazine()));
                typewrite(format!("\n\x1b[31m{}\x1b[0m live.", live));
                sleep(Duration::from_millis(500));
                typewrite(format!(" \x1b[33m{}\x1b[0m blank{}.\n\n", blanks, if blanks == 1 { "" } else { "s" }));
                sleep(Duration::from_millis(500));
            }
            _ => {
                typewrite(print_magazine(self.bullets, self.shuffled_magazine()));
                if round == 0 { typewrite("\nYou know the drill.\n".to_string()); }
                println!();
            }
        }
        print!("     ");

        let mut lines = io::stdin().lock().lines();

        let mut careful = self.player_lives == 1;
        while !self.done() {
            println!("{self}");
            let hit = if self.players_turn {
                print!("\x1b[90m[d/y]\x1b[0m ");
                io::stdout().flush().unwrap();
                let line = lines.next().expect("couldn't read stdin").unwrap();
                let r = match line.trim() {
                    "d" => self.shoot(false).unwrap(),
                    "y" => self.shoot(true).unwrap(),
                    _ => {
                        print!("\x1b[F\x1b[2K\x1b[F\x1b[5C");
                        continue;
                    }
                };
                print!("\x1b[F\x1b[2K\r");
                r
            } else {
                self.shoot(false).unwrap()
            };

            if !careful && self.player_lives == 1 {
                sleep(Duration::from_millis(500));
                typewrite("\n\x1b[90mCareful now...\x1b[0m".to_string());
                sleep(Duration::from_millis(900));
                print!("\x1b[2K\x1b[F");
                careful = true;
            }

            if hit {
                typewrite("\x1b[31mHIT \x1b[0m ".to_string());
            } else {
                typewrite("\x1b[32mMISS\x1b[0m ".to_string());
            }
        }
        println!();
        let result = if self.player_lives == 0 {
            typewrite("\n\x1b[31mI WIN\x1b[0m\n".to_string());
            DealerWins
        } else if self.dealer_lives == 0 {
            typewrite("\n\x1b[32mYOU WIN\x1b[0m\n".to_string());
            PlayerWins
        } else {
            Continue
        };
        // println!("\n{magazine} {self}\n\n");
        result
    }

    pub fn reload(&mut self) {
        (self.bullets, self.magazine) = create_magazine(0);
        self.players_turn = true;
    }
}

impl fmt::Display for Round {
    //noinspection RsConstantConditionIf
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} \x1b[33m⚡ {}{}\x1b[0m",
            if self.players_turn { "\x1b[36m" } else { "" },
            self.player_lives,
            if !self.players_turn { "\x1b[31m" } else { "\x1b[0m" },
            self.dealer_lives)
    }
}

fn print_magazine(bullets: u8, magazine: u8) -> String {
    let mut result = String::with_capacity((bullets * 7 + 4) as usize);
    for i in 0..bullets {
        result.push_str(if (magazine >> i) & 1 == 1 { "\x1b[31m█ " } else { "\x1b[34m█ " });
    }
    result.push_str("\x1b[0m");
    result
}

fn create_magazine(mut bullets: u8) -> (u8, u8) {
    let mut rng = thread_rng();
    if bullets == 0 {
        bullets = rng.gen_range(2..=8);
    }
    let live_rounds = rng.gen_range(1..=((bullets + 1) / 2));
    let pos = rand::seq::index::sample(&mut rng, bullets as usize, live_rounds as usize).into_vec();
    let mut magazine: u8 = 0;

    for p in pos {
        magazine |= 1 << p;
    }

    (bullets, magazine)
}

pub fn create_first_round(mut lives: u8, bullets: u8) -> Round {
    if lives == 0 {
        let mut rng = thread_rng();
        lives = rng.gen_range(2..=6);
    }
    create_round(lives, lives, bullets)
}

fn create_round(player_lives: u8, dealer_lives: u8, bullets: u8) -> Round {
    let (bullets, magazine) = create_magazine(bullets);

    Round {
        bullets,
        players_turn: true,
        magazine,
        dealer_lives,
        player_lives,
    }
}

fn run_stage(stage: u8) -> RoundResult {
    let mut round = if stage == 0 { create_first_round(2, 3) } else { create_first_round(0, 0) };
    let mut result = round.play(stage, 0);
    let mut i = 0;
    while result == Continue {
        i += 1;
        round.reload();
        result = round.play(stage, i);
    }
    result
}

pub fn play() {
    for i in 0..=2 {
        println!("\n\x1b[35mStage {}\x1b[0m\n", ["I", "II", "☠️"][i]);
        if run_stage(i as u8) == DealerWins {
            break;
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum RoundResult {
    DealerWins,
    PlayerWins,
    Continue,
}

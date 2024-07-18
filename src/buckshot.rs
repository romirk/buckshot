use std::{fmt, io};
use std::fmt::Formatter;
use std::io::{BufRead, Lines, StdinLock, Write};
use std::thread::sleep;
use std::time::Duration;

use rand::{Rng, thread_rng};

use crate::dealer::{decide, think};
use crate::types::{Action, Item, Items, VolleyExport, VolleyResult};
use crate::types::Action::{Murder, Suicide, Use};
use crate::types::Item::{MagnifyingGlass, Nothing};
use crate::types::Round::{Blank, Live, Unknown};
use crate::types::VolleyResult::{Continue, DealerWins, PlayerWins};
use crate::typewriter::{peek, typewrite};

// #[derive(Debug)]
// struct Player {
//     pub lives: u8,
//     // 4 bit item counts in order of Item
//     // items: u64,
// }


pub struct Volley {
    players_turn: bool,

    bullets: u8,
    loaded: u8,
    // 1 bit per bullet, 1 = live
    magazine: u8,
    history: u8,
    shot: u8,

    max_lives: u8,
    dealer_lives: u8,

    player_lives: u8,
    dealer_items: Items,
    player_items: Items,
}


impl Volley {
    fn done(&self) -> bool {
        self.bullets == 0 || self.player_lives == 0 || self.dealer_lives == 0
    }

    pub(crate) fn live(&self) -> u8 {
        self.magazine.count_ones() as u8
    }

    pub(crate) fn shoot(&mut self, suicide: bool) -> Result<bool, &'static str> {
        if self.bullets == 0 {
            return Err("No bullets.");
        }

        if self.player_lives == 0 || self.dealer_lives == 0 {
            return Err("Somebody already died.");
        }


        let live = self.rack();

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

    fn rack(&mut self) -> bool {
        self.history <<= 1;
        self.history &= self.magazine & 1;
        self.shot += 1;
        let live = (self.magazine & 1) != 0;
        self.magazine >>= 1;
        self.bullets -= 1;
        live
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

    fn peek(&self) -> bool {
        self.magazine & 1 == 1
    }

    pub fn export(&self) -> VolleyExport {
        VolleyExport {
            bullets: self.bullets,
            live: self.live(),
            loaded: self.loaded,
            max_lives: self.max_lives,
            players_turn: self.players_turn,
            player_lives: self.player_lives,
            dealer_lives: self.dealer_lives,
            player_items: self.player_items,
            dealer_items: self.dealer_items,
            current_bullet: Unknown,
            history: self.history,
            shot: self.shot,
        }
    }

    fn use_player_item(&mut self, item: Item) -> bool {
        match item {
            Nothing => {
                true
            }
            Item::Beer => {
                if self.player_items.beer > 0 {
                    self.player_items.beer -= 1;
                    self.rack();
                    return true;
                }
                false
            }
            Item::Cigarettes => {
                if self.player_items.cigarettes == 0 {
                    return false;
                }
                self.player_items.cigarettes -= 1;
                if self.player_lives < self.max_lives {
                    self.player_lives += 1;
                }
                true
            }
            Item::MagnifyingGlass => {
                if self.player_items.magnifying_glass == 0 {
                    return false;
                }
                self.player_items.magnifying_glass -= 1;
                let live = self.peek();
                let bullet = if live { Live } else { Blank };
                peek(bullet);
                print!("\r\x1b[2K  🔍 ");
                true
            }
        }
    }

    fn use_dealer_item(&mut self, item: Item) -> VolleyExport {
        match item {
            Nothing => { self.export() }
            Item::Beer => {
                if self.dealer_items.beer == 0 {
                    panic!("Dealer used {item} without owning it!");
                }
                self.dealer_items.beer -= 1;
                self.rack();
                self.export()
            }
            Item::Cigarettes => {
                if self.dealer_items.cigarettes == 0 {
                    panic!("Dealer used {item} without owning it!");
                }
                self.dealer_items.cigarettes -= 1;
                if self.dealer_lives < self.max_lives {
                    self.dealer_lives += 1;
                }
                self.export()
            }
            Item::MagnifyingGlass => {
                if self.dealer_items.magnifying_glass == 0 {
                    panic!("Dealer used {item} without owning it!");
                }
                self.dealer_items.magnifying_glass -= 1;
                let live = self.peek();
                let bullet = if live { Live } else { Blank };
                peek(Unknown);
                let mut export = self.export();
                export.current_bullet = bullet;
                export
            }
        }
    }

    pub fn play(&mut self, stage: u8, round: u8) -> VolleyResult {
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
                let action = self.input(&mut lines);
                print!("\x1b[F\x1b[2K\r");
                match action {
                    Murder => {
                        self.shoot(false).unwrap()
                    }
                    Suicide => {
                        self.shoot(true).unwrap()
                    }
                    Use => {
                        let item = self.item_input(&mut lines);
                        print!("\x1b[F\x1b[2K\r");
                        self.use_player_item(item);
                        continue;
                    }
                }
            } else {
                // TODO dealer AI goes here
                let mut export = self.export();
                let mut item = think(export);
                while item != Nothing {
                    export = self.use_dealer_item(item);
                    item = think(export);
                }
                self.shoot(decide(export)).unwrap()
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
        println!("{self}");

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

    fn item_input(&mut self, lines: &mut Lines<StdinLock>) -> Item {
        print!("\x1b[90m[m/b]\x1b[0m ");
        io::stdout().flush().unwrap();
        let line = lines.next().expect("couldn't read stdin").unwrap();
        match line.trim() {
            "m" => MagnifyingGlass,
            "b" => Nothing,
            _ => {
                print!("\x1b[F\x1b[2K\rmagnifying glass/back ");
                self.item_input(lines)
            }
        }
    }
    fn shot_input(&mut self, lines: &mut Lines<StdinLock>) -> bool {
        print!("\x1b[90m[d/y]\x1b[0m ");
        io::stdout().flush().unwrap();
        let line = lines.next().expect("couldn't read stdin").unwrap();
        match line.trim() {
            "d" => false,
            "y" => true,
            _ => {
                print!("\x1b[F\x1b[2K\rdealer/yourself ");
                self.shot_input(lines)
            }
        }
    }
    fn input(&mut self, lines: &mut Lines<StdinLock>) -> Action {
        print!("\x1b[90m[d/y/i]\x1b[0m ");
        io::stdout().flush().unwrap();
        let line = lines.next().expect("couldn't read stdin").unwrap();
        match line.trim() {
            "d" => Murder,
            "y" => Suicide,
            "i" => Use,
            _ => {
                print!("\x1b[F\x1b[2K\rdealer/yourself/item ");
                self.input(lines)
            }
        }
    }

    pub fn reload(&mut self) {
        (self.bullets, self.magazine) = create_magazine(0);
        self.players_turn = true;
    }
}

impl fmt::Display for Volley {
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

pub fn create_first_round(mut lives: u8, bullets: u8) -> Volley {
    if lives == 0 {
        let mut rng = thread_rng();
        lives = rng.gen_range(2..=6);
    }
    let (bullets, magazine) = create_magazine(bullets);

    Volley {
        players_turn: true,
        bullets,
        loaded: magazine.count_ones() as u8,
        magazine,
        history: 0,
        shot: 0,
        dealer_lives: lives,
        player_lives: lives,
        player_items: Items {
            beer: 0,
            cigarettes: 0,
            magnifying_glass: 3,
        },
        dealer_items: Items {
            beer: 0,
            cigarettes: 0,
            magnifying_glass: 3,
        },
        max_lives: lives,
    }
}

fn run_stage(stage: u8) -> VolleyResult {
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


use std::fmt;
use std::fmt::Formatter;

use rand::{Rng, thread_rng};

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
    pub fn done(&self) -> bool {
        self.bullets == 0 || self.player_lives == 0 || self.dealer_lives == 0
    }

    pub fn live(&self) -> u8 {
        self.magazine.count_ones() as u8
    }

    pub fn players_turn(&self) -> bool {
        self.players_turn
    }

    pub fn lives(&self) -> [u8; 2] {
        [self.player_lives, self.dealer_lives]
    }

    pub fn bullets(&self) -> u8 {
        self.bullets
    }

    pub fn shoot(&mut self, suicide: bool) -> Result<bool, &'static str> {
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

    pub fn debug_magazine(&self) -> String {
        format!("{:08b}", self.magazine)
    }
}

impl fmt::Display for Round {
    //noinspection RsConstantConditionIf
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} ⚡ {}{}",
            if self.players_turn { "*" } else { " " },
            self.player_lives,
            self.dealer_lives,
            if !self.players_turn { "*" } else { " " })
    }
}

pub fn create_round() -> Round {
    let mut rng = thread_rng();
    let bullets: u8 = rng.gen_range(2..=8);
    let live_rounds = rng.gen_range(1..=((bullets + 1) / 2));
    let lives = rng.gen_range(1..=live_rounds);
    let pos = rand::seq::index::sample(&mut rng, bullets as usize, live_rounds as usize).into_vec();
    let mut magazine: u8 = 0;

    for p in pos {
        magazine |= 1 << p;
    }

    Round {
        bullets,
        players_turn: true,
        magazine,
        dealer_lives: lives,
        player_lives: lives,
    }
}

use std::fmt;
use std::fmt::Formatter;

use rand::{Rng, thread_rng};

use crate::error::BuckshotError;
use crate::error::BuckshotError::ValueError;

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

    pub fn shoot(&mut self, suicide: bool) -> Result<(), &'static str> {
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
                return Ok(());
            }
        } else {
            if (suicide && !self.players_turn) || (!suicide && self.players_turn) {
                self.dealer_lives -= 1;
            } else {
                self.player_lives -= 1;
            }
        }
        self.players_turn = !self.players_turn;
        Ok(())
    }
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:08b} {}{:?} ⚡ {:?}{}", self.magazine, if self.players_turn { "*" } else { " " }, self.player_lives, self.dealer_lives, if !self.players_turn { "*" } else { " " })
    }
}

pub fn create_round(lives: u8) -> Result<Round, BuckshotError> {
    if lives < 2 || lives > 6 {
        return Err(ValueError);
    }

    let mut rng = thread_rng();
    let bullets: u8 = rng.gen_range(2..=8);
    let live = rng.gen_range(1..=((bullets + 1) / 2));
    let pos = rand::seq::index::sample(&mut rng, bullets as usize, live as usize).into_vec();
    let mut magazine: u8 = 0;

    for p in pos {
        magazine |= 1 << p;
    }

    Ok(Round {
        bullets,
        players_turn: true,
        magazine,
        dealer_lives: lives,
        player_lives: lives,
    })
}

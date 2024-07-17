use std::cmp::PartialEq;

#[derive(Eq, PartialEq)]
enum Item {
    Adrenaline,
    Beer,
    BurnerPhone,
    Cigarettes,
    ExpiredMedicine,
    HandSaw,
    Handcuffs,
    Inverter,
    MagnifyingGlass,
}


// #[derive(Debug)]
// struct Player {
//     pub lives: u8,
//     // 4 bit item counts in order of Item
//     // items: u64,
// }

#[derive(Debug)]
pub struct Round {
    pub bullets: u8,
    pub players_turn: bool,

    // 1 bit per bullet, 1 = live
    pub magazine: u8,

    pub dealer_lives: u8,
    pub player_lives: u8
}

impl Round {
    pub fn shoot(&mut self, suicide: bool) -> Result<(), &str> {
        if (self.bullets == 0) {
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

pub fn create_round() -> Round {
    Round {
        bullets: 6,
        players_turn: true,
        magazine: 0b010110,
        dealer_lives: 3,
        player_lives: 3
    }
}

use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum Round {
    Unknown,
    Blank,
    Live,
}

#[derive(PartialEq)]
pub enum VolleyResult {
    DealerWins,
    PlayerWins,
    Continue,
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
pub enum Item {
    Nothing,
    // Adrenaline,
    Beer,
    // BurnerPhone,
    Cigarettes,
    // ExpiredMedicine,
    // HandSaw,
    // Handcuffs,
    // Inverter,
    MagnifyingGlass,
}


#[derive(Clone, Copy)]
pub struct Items {
    // adrenaline: u8,
    pub beer: u8,
    // burner_phone: u8,
    pub cigarettes: u8,
    // expired_meds: u8,
    // hand_saw: u8,
    // handcuffs: u8,
    // inverter: u8,
    pub magnifying_glass: u8,
}

#[derive(Clone, Copy)]
pub struct VolleyExport {
    pub bullets: u8,
    pub live: u8,
    pub loaded: u8,
    pub max_lives: u8,
    pub players_turn: bool,
    pub player_lives: u8,
    pub dealer_lives: u8,
    pub player_items: Items,
    pub dealer_items: Items,
    pub current_bullet: Round,
    pub history: u8,
    pub shot: u8
}


impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
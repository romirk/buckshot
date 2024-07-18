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

#[derive(PartialEq)]
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
pub struct VolleyExport {
    pub bullets: u8,
    pub players_turn: bool,
    pub player_lives: u8,
    pub dealer_lives: u8,
    pub current_bullet: Round,
    pub shot: u8,
}

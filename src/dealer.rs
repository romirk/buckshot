use crate::types::{Item, VolleyExport};
use crate::types::Item::{Cigarettes, MagnifyingGlass, Nothing};
use crate::types::Round::{Blank, Unknown};

fn guaranteed_shot(volley: VolleyExport) -> bool {
    let live_shot = volley.history.count_ones() as u8;
    let live_remaining = volley.loaded - live_shot;

    if live_remaining == 0 {
        return true;
    }

    if volley.bullets == 1 {
        return true;
    }
    return false;
}

pub fn think(volley: VolleyExport) -> Item {
    if volley.dealer_lives < volley.max_lives && volley.dealer_items.cigarettes > 0 {
        return Cigarettes;
    }
    if volley.current_bullet == Unknown {
        if guaranteed_shot(volley) {
            return Nothing;
        }
        if volley.dealer_items.magnifying_glass > 0 {
            return MagnifyingGlass;
        }
    }
    Nothing
}

pub fn decide(volley: VolleyExport) -> bool {
    if volley.current_bullet != Unknown {
        return volley.current_bullet == Blank;
    }
    let live_shot = volley.history.count_ones() as u8;
    let live_remaining = volley.loaded - live_shot;

    if live_remaining == 0 {
        return true;
    }

    if volley.bullets == 1 {
        return false;
    }

    return false;
}

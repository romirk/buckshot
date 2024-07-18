use crate::types::{Item, VolleyExport};
use crate::types::Item::{MagnifyingGlass, Nothing};
use crate::types::Round::{Blank, Unknown};

pub fn think(volley: VolleyExport) -> Item {
    if volley.current_bullet == Unknown {
        return MagnifyingGlass;
    }
    Nothing
}

pub fn decide(volley: VolleyExport) -> bool {
    volley.current_bullet == Blank
}

use std::fmt;

use player::Player;

pub mod base;
pub mod ship;
pub mod outpost;

pub enum Faction {
    Unaligned
}

pub enum CardType {
    Ship
}


pub trait Card {
    fn play(&self, player: &mut Player) -> Player;
    // TODO: implement these
    // fn discard(&self, player: &Player) -> Player;
    // fn scrap(&self, player: &Player) -> Player;
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            CardType::Ship => "Ship"
        };
        write!(f, "{}", name)
    }
}

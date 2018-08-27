use std::fmt;

pub mod base;
pub mod ship;
pub mod outpost;
pub mod targetable;

pub enum Faction {
    MachineCult,
    Unaligned
}

pub enum CardType {
    Outpost,
    Ship
}

pub enum ShipType {
    Scout
}


// pub trait Card: fmt::Display {
//     fn play(&self, player: &mut Player);
//     // TODO: implement these
//     // fn discard(&self, player: &Player) -> Player;
//     // fn scrap(&self, player: &Player) -> Player;
// }

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            CardType::Ship => "Ship",
            CardType::Outpost => "Outpost"
        };
        write!(f, "{}", name)
    }
}

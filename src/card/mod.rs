use std::fmt;

pub mod base;
pub mod ship;
pub mod outpost;

pub enum Faction {
    Blob,
    MachineCult,
    TradeFederation,
    StarEmpire,
    Unaligned,
}

pub enum CardType {
    Ship,
    Base,
    Outpost,
}

pub struct Card {
    pub card_type: CardType,
    pub faction: Faction,
    pub name: String,
    pub cost: i32,
    pub combat: i32,
    pub trade: i32,
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            CardType::Ship => "Ship",
            CardType::Base => "Base",
            CardType::Outpost => "Outpost",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>\n", self.card_type, self.name)
    }
}

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
    Explorer,
    Scout,
    Viper,
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            CardType::Ship => "Ship",
            CardType::Outpost => "Outpost"
        };
        write!(f, "{}", name)
    }
}

pub struct Card {
    pub card_type: CardType,
    pub combat: i32,
    pub cost: i32,
    pub faction: Faction,
    pub name: String,
    pub ship_type: ShipType,
    pub trade: i32,
}

impl Card {
    pub fn scout() -> Card {
        Card {
            card_type: CardType::Ship,
            combat: 0,
            cost: 0,
            faction: Faction::Unaligned,
            name: String::from("Scout"),
            trade: 1,
            ship_type: ShipType::Scout,
        }
    }

    pub fn explorer() -> Card {
        Card {
            card_type: CardType::Ship,
            combat: 0,
            cost: 2,
            faction: Faction::Unaligned,
            name: String::from("Explorer"),
            trade: 2,
            ship_type: ShipType::Explorer,
        }
    }

    pub fn viper() -> Card {
        Card {
            card_type: CardType::Ship,
            combat: 1,
            cost: 0,
            faction: Faction::Unaligned,
            name: String::from("Viper"),
            trade: 0,
            ship_type: ShipType::Viper,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>\n", self.card_type, self.name)
    }
}

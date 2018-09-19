use std::fmt;

pub mod base;
pub mod ship;
pub mod outpost;
pub mod targetable;

#[derive(Clone)]
#[derive(PartialEq)]
pub enum Faction {
    Blob,
    MachineCult,
    Unaligned,
}

impl Default for Faction {
    fn default() -> Faction { Faction::Unaligned }
}

#[derive(Clone)]
pub enum CardType {
    NoCardType,
    Outpost,
    Ship,
    Base
}

impl Default for CardType {
    fn default() ->  CardType { CardType::NoCardType }
}

#[derive(Clone)]
pub enum ShipType {
    BattleBlob,
    Explorer,
    NoShipType,
    Scout,
    Viper,
}

impl Default for ShipType {
    fn default() ->  ShipType { ShipType::NoShipType }
}

#[derive(Clone)]
pub enum OutpostType {
    BattleStation,
    NoOutpostType,
}

impl Default for OutpostType {
    fn default() ->  OutpostType { OutpostType::NoOutpostType }
}

#[derive(Clone)]
pub enum BaseType {
    TheHive,
    NoBaseType,
}

impl Default for BaseType {
    fn default() ->  BaseType { BaseType::NoBaseType }
}

impl fmt::Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Faction::Blob => "Blob",
            Faction::MachineCult => "Machine Cult",
            Faction::Unaligned => "Unaligned",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            CardType::Ship => "Ship",
            CardType::Outpost => "Outpost",
            CardType::Base => "Base",
            CardType::NoCardType => "NoCardType",
        };
        write!(f, "{}", name)
    }
}


#[derive(Clone)]
#[derive(Default)]
pub struct Card {
    pub base_type: BaseType,
    pub card_type: CardType,
    pub combat: i32,
    pub cost: i32,
    pub faction: Faction,
    pub health: i32,
    pub name: String,
    pub ship_type: ShipType,
    pub outpost_type: OutpostType,
    pub trade: i32,
    pub scrappable: bool,
    pub has_used_ally_ability: bool
}

impl Card {
    pub fn none() -> Card {
        Card{ ..Default::default() }
    }

    pub fn battle_blob() -> Card {
        Card {
            card_type: CardType::Ship,
            cost: 6,
            combat: 8,
            faction: Faction::Blob,
            name: String::from("BattleBlob"),
            ship_type: ShipType::BattleBlob,
            ..Default::default()
        }
    }

    pub fn scout() -> Card {
        Card {
            card_type: CardType::Ship,
            name: String::from("Scout"),
            trade: 1,
            ship_type: ShipType::Scout,
            ..Default::default()
        }
    }

    pub fn explorer() -> Card {
        Card {
            card_type: CardType::Ship,
            cost: 2,
            name: String::from("Explorer"),
            ship_type: ShipType::Explorer,
            trade: 2,
            scrappable: true,
            ..Default::default()
        }
    }

    pub fn viper() -> Card {
        Card {
            card_type: CardType::Ship,
            combat: 1,
            name: String::from("Viper"),
            ship_type: ShipType::Viper,
            ..Default::default()
        }
    }

    pub fn battle_station() -> Card {
        Card {
            card_type: CardType::Outpost,
            cost: 3,
            faction: Faction::MachineCult,
            health: 5,
            name: String::from("Battle Station"),
            outpost_type: OutpostType::BattleStation,
            scrappable: true,
            ..Default::default()
        }
    }

    pub fn the_hive() -> Card {
        Card {
            card_type: CardType::Base,
            cost: 5,
            faction: Faction::Blob,
            health: 5,
            name: String::from("The Hive"),
            base_type: BaseType::TheHive,
            ..Default::default()
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>\n", self.card_type, self.name)
    }
}

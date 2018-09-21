use std::fmt;

pub mod base;
pub mod ship;
pub mod outpost;
pub mod targetable;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Faction {
    Blob,
    MachineCult,
    StarEmpire,
    Unaligned,
}

impl Default for Faction {
    fn default() -> Faction { Faction::Unaligned }
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Type {
    NoCardType,
    Outpost,
    Ship,
    Base
}

impl Default for Type {
    fn default() ->  Type { Type::NoCardType }
}

impl fmt::Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Faction::Blob => "Blob",
            Faction::MachineCult => "Machine Cult",
            Faction::StarEmpire => "Star Empire",
            Faction::Unaligned => "Unaligned",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Type::Ship => "Ship",
            Type::Outpost => "Outpost",
            Type::Base => "Base",
            Type::NoCardType => "NoCardType",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone)]
#[derive(Default)]
#[derive(Debug)]
pub struct Card {
    pub base_type: base::Type,
    pub card_type: Type,
    pub combat: i32,
    pub cost: i32,
    pub faction: Faction,
    pub health: i32,
    pub name: String,
    pub ship_type: ship::Type,
    pub outpost_type: outpost::Type,
    pub trade: i32,
    pub scrappable: bool,
    pub has_ally_ability: bool,
    pub has_used_ally_ability: bool
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>\n", self.card_type, self.name)
    }
}

use std::fmt;

use card::Card;
use card::CardType;
use card::Faction;
use player::Player;

pub struct Scout {
    pub card_type: CardType,
    pub combat: i32,
    pub cost: i32,
    pub faction: Faction,
    pub name: String,
    pub trade: i32,
}

impl Scout {
    pub fn new() -> Scout {
        Scout{
            card_type: CardType::Ship,
            combat: 0,
            cost: 0,
            faction: Faction::Unaligned,
            name: String::from("Scout"),
            trade: 1,
        }
    }
}

impl Card for Scout {
    fn play(&self, player: &mut Player) {
        player.trade += 1;
    }
}

impl fmt::Display for Scout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>\n", self.card_type, self.name)
    }
}

pub struct Viper {
    pub card_type: CardType,
    pub combat: i32,
    pub cost: i32,
    pub faction: Faction,
    pub name: String,
    pub trade: i32,
}

impl Viper {
    pub fn new() -> Viper {
        Viper{
            card_type: CardType::Ship,
            combat: 1,
            cost: 0,
            faction: Faction::Unaligned,
            name: String::from("Viper"),
            trade: 0,
        }
    }
}

impl Card for Viper {
    fn play(&self, player: &mut Player) {
        player.combat += 1;
    }
}

impl fmt::Display for Viper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>\n", self.card_type, self.name)
    }
}

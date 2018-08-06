use std::fmt;

use card::Card;
use card::CardType;
use card::Faction;
use player::Player;

pub struct Scout {
    pub card_type: CardType,
    pub faction: Faction,
    pub name: String,
    pub cost: i32,
    pub combat: i32,
    pub trade: i32,
}

impl Scout {
    pub fn new() -> Scout {
        Scout{
            name: String::from("Scout"),
            cost: 0,
            combat: 0,
            trade: 1,
            card_type: CardType::Ship,
            faction: Faction::Unaligned
        }
    }
}

impl Card for Scout {
    fn play(&self, player: &Player) -> Player {
        let mut p = Player::new(&player.name);
        p.trade += 1;
        p
    }
}

impl fmt::Display for Scout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>\n", self.card_type, self.name)
    }
}

use std::fmt;

use card::Card;
use card::CardType;
use card::Faction;
use card::targetable::Targetable;

use player::Player;

pub struct BattleStation {
    pub card_type: CardType,
    pub combat: i32,
    pub cost: i32,
    pub faction: Faction,
    pub health: i32,
    pub name: String,
    pub trade: i32
}

impl BattleStation {
    pub fn new() -> BattleStation {
        BattleStation{
            card_type: CardType::Outpost,
            combat: 0,
            cost: 3,
            faction: Faction::MachineCult,
            health: 5,
            name: String::from("Battle Station"),
            trade: 0,
        }
    }
}

impl Card for BattleStation {
    fn play(&self, player: &mut Player) {
        // TODO: figure out how to push self onto player's bases list
        player.bases.push(Box::new(BattleStation::new()));
    }
}

impl Targetable for BattleStation {
    fn is_targetable(&self) -> bool {
        true
    }

    fn receive_combat(&mut self, combat: i32) {
        if combat >= self.health {
            // TODO: move from player.bases to player.discard
        }
    }
}

impl fmt::Display for BattleStation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>\n", self.card_type, self.name)
    }
}

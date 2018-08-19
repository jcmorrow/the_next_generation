use std::fmt;

use card::Card;
use card::CardType;
use card::Faction;
use card::targetable::Targetable;

use player::Player;

pub struct BattleStation {
    pub card_type: CardType,
    pub faction: Faction,
    pub name: String,
    pub cost: i32,
    pub combat: i32,
    pub trade: i32,
    pub health: i32
}

impl BattleStation {
    pub fn new() -> BattleStation {
        BattleStation{
            name: String::from("Battle Station"),
            cost: 3,
            combat: 0,
            trade: 0,
            health: 5,
            card_type: CardType::Outpost,
            faction: Faction::MachineCult
        }
    }
}

impl Card for BattleStation {
    fn play(&self, mut player: Player) -> Player {
        player.bases.push(Box::new(BattleStation::new()));
        player
    }
}

impl Targetable for BattleStation {
    fn is_targetable(&self) -> bool {
        true
    }

    fn process_attack(&self, player: Player, combat: i32) -> Player {
        if combat >= self.health {
            // TODO: move from player.bases to player.discard
        }
        player
    }
}

impl fmt::Display for BattleStation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>\n", self.card_type, self.name)
    }
}

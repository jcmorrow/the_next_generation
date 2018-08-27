use std::fmt;
use card;
use card::CardType;
use card::ShipType;
use card::Faction;
use player::Player;

pub struct Card {
    pub card_type: CardType,
    pub combat: i32,
    pub cost: i32,
    pub faction: Faction,
    pub name: String,
    pub ship_type: card::ShipType,
    pub trade: i32,
}

impl Card {
    pub fn scout() -> Card {
        Card{
            card_type: CardType::Ship,
            combat: 0,
            cost: 0,
            faction: Faction::Unaligned,
            name: String::from("Scout"),
            trade: 1,
            ship_type: ShipType::Scout,
        }
    }
}

pub struct PlayEvent<'a> {
    player: &'a Player,
    card: &'a Card,
}

impl<'a> PlayEvent<'a> {
    pub fn new(card: &'a Card, player: &'a mut Player) -> PlayEvent<'a> {
        PlayEvent {
            card: card,
            player: player,
        }
    }

    pub fn play(&self) {
        // match card.ship_type {
        //     ShipType::Scout => { player.trade += 1; }
        // }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}: {}>\n", self.card_type, self.name)
    }
}

// pub struct Viper {
//     pub card_type: CardType,
//     pub combat: i32,
//     pub cost: i32,
//     pub faction: Faction,
//     pub name: String,
//     pub trade: i32,
// }

// impl Viper {
//     pub fn new() -> Viper {
//         Viper{
//             card_type: CardType::Ship,
//             combat: 1,
//             cost: 0,
//             faction: Faction::Unaligned,
//             name: String::from("Viper"),
//             trade: 0,
//         }
//     }
// }

// impl Card for Viper {
//     fn play(&self, player: &mut Player) {
//         player.combat += 1;
//     }
// }

// impl fmt::Display for Viper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "<{}: {}>\n", self.card_type, self.name)
//     }
// }

// pub struct Explorer {
//     pub card_type: CardType,
//     pub combat: i32,
//     pub cost: i32,
//     pub faction: Faction,
//     pub name: String,
//     pub trade: i32,
// }

// impl Explorer {
//     pub fn new() -> Explorer {
//         Explorer{
//             card_type: CardType::Ship,
//             combat: 0,
//             cost: 2,
//             faction: Faction::Unaligned,
//             name: String::from("Explorer"),
//             trade: 2,
//         }
//     }
// }

// impl Card for Explorer {
//     fn play(&self, player: &mut Player) {
//         player.trade += self.trade;
//     }
// }

// impl fmt::Display for Explorer {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "<{}: {}>\n", self.card_type, self.name)
//     }
// }

use card::Card;
use card::CardType;
use card::Faction;
use card::BaseType;
use choice::Choice;

// pub fn blob_world() -> Card {
//     Card {
//         abilities: vec!(Choice::Or(
//             Box::new(Choice::GainAttack(5)),
//             Box::new(Choice::BlobDraw(0)),
//             true
//         )),
//         card_type: CardType::Base,
//         cost: 8,
//         faction: Faction::Blob,
//         health: 7,
//         name: String::from("Blob World"),
//         base_type: BaseType::TheHive,
//         ..Default::default()
//     }
// }
//
// pub fn the_hive() -> Card {
//     Card {
//         card_type: CardType::Base,
//         cost: 5,
//         faction: Faction::Blob,
//         health: 5,
//         name: String::from("The Hive"),
//         base_type: BaseType::TheHive,
//         ..Default::default()
//     }
// }

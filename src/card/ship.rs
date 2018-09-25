use card::Card;
use card::CardType;
use card::Faction;
use card::ShipType;
use choice::Choice;
use effect::Effect;

// pub fn battle_blob() -> Card {
//     Card {
//         abilities: vec!(Choice::GainAttack(8)),
//         card_type: CardType::Ship,
//         cost: 6,
//         faction: Faction::Blob,
//         name: String::from("Battle Blob"),
//         ship_type: ShipType::BattleBlob,
//         ..Default::default()
//     }
// }
//
// pub fn battle_pod() -> Card {
//     Card {
//         abilities: vec!(Choice::ScrapFromTradeRow(0), Choice::GainAttack(4)),
//         ally_abilities: vec!(Choice::GainAttack(2)),
//         card_type: CardType::Ship,
//         cost: 2,
//         faction: Faction::Blob,
//         name: String::from("Battle Pod"),
//         ship_type: ShipType::BattlePod,
//         ..Default::default()
//     }
// }
//
// pub fn blob_carrier() -> Card {
//     Card {
//         abilities: vec!(Choice::GainAttack(7)),
//         ally_abilities: vec!(Choice::AcquireFromTradeRow(0)),
//         card_type: CardType::Ship,
//         cost: 6,
//         faction: Faction::Blob,
//         name: String::from("Blob Carrier"),
//         ship_type: ShipType::BlobCarrier,
//         ..Default::default()
//     }
// }
//
// pub fn blob_destroyer() -> Card {
//     Card {
//         ally_abilities: vec!(
//                             Choice::AndOr(
//                                 Box::new(Choice::DestroyBase(0, 0)),
//                                 Box::new(Choice::ScrapFromTradeRow(0)),
//                                 false,
//                                 false
//                             )
//                         ),
//         abilities: vec!(Choice::GainAttack(6)),
//         card_type: CardType::Ship,
//         cost: 4,
//         faction: Faction::Blob,
//         name: String::from("Blob Destoyer"),
//         ship_type: ShipType::BlobDestroyer,
//         ..Default::default()
//     }
// }
//
pub fn explorer() -> Card {
    Card {
        abilities: vec!(Choice::GainTrade(2)),
        card_type: CardType::Ship,
        cost: 2,
        name: String::from("Explorer"),
        scrap_abilities: vec!(Choice::GainAttack(2)),
        ship_type: ShipType::Explorer,
        ..Default::default()
    }
}
//
// pub fn imperial_fighter() -> Card {
//     Card {
//         abilities: vec!(Choice::GainAttack(2), Choice::DiscardAttack(0)),
//         ally_abilities: vec!(Choice::GainAttack(2)),
//         card_type: CardType::Ship,
//         cost: 1,
//         faction: Faction::StarEmpire,
//         name: String::from("Imperial Fighter"),
//         ship_type: ShipType::ImperialFighter,
//         ..Default::default()
//     }
// }
//
pub fn scout() -> Card {
    Card {
        card_type: CardType::Ship,
        effects: vec!(Effect::GainTrade(1)),
        name: String::from("Scout"),
        ship_type: ShipType::Scout,
        ..Default::default()
    }
}
//
// pub fn trade_bot() -> Card {
//     Card {
//         abilities: vec!(
//             Gain::Trade(1),
//             Choice::Or(
//                 Box::new(Choice::ScrapHand(0)),
//                 Box::new(Choice::ScrapDiscard(0)),
//                 true
//             )),
//         ally_abilities: vec!(Gain::Attack(2)),
//         card_type: CardType::Ship,
//         cost: 1,
//         faction: Faction::MachineCult,
//         name: String::from("Trade Bot"),
//         ship_type: ShipType::TradeBot,
//         ..Default::default()
//     }
// }

pub fn viper() -> Card {
    Card {
        // abilities: vec!(Choice::GainAttack(1)),
        card_type: CardType::Ship,
        effects: vec!(Effect::GainCombat(1)),
        name: String::from("Viper"),
        ship_type: ShipType::Viper,
        ..Default::default()
    }
}

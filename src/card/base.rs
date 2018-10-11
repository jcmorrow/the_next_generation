use card::Card;
use card::CardType;
use card::Faction;
use card::BaseType;
use choice::Choice;
use effect::Effect;

pub fn blob_wheel() -> Card {
    Card {
        card_type: CardType::Base,
        cost: 3,
        faction: Faction::Blob,
        health: 5,
        name: String::from("Blob Wheel"),
        scrap_effects: vec!(Effect::GainTrade(3)),
        base_type: BaseType::BlobWheel,
        ..Default::default()
    }
}

pub fn blob_world() -> Card {
    Card {
        abilities: vec!(Choice::Or(
            Box::new(Choice::GainCombat(5)),
            Box::new(Choice::BlobDraw(0)),
            true
        )),
        card_type: CardType::Base,
        cost: 8,
        faction: Faction::Blob,
        health: 7,
        name: String::from("Blob World"),
        base_type: BaseType::BlobWorld,
        ..Default::default()
    }
}

// pub fn fleet_hq() -> Card {
//      TODO
// }

pub fn the_hive() -> Card {
    Card {
        ally_effects: vec!(Effect::Draw),
        card_type: CardType::Base,
        cost: 5,
        faction: Faction::Blob,
        health: 5,
        name: String::from("The Hive"),
        base_type: BaseType::TheHive,
        ..Default::default()
    }
}

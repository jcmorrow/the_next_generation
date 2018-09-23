use card::Card;
use card::CardType;
use card::Faction;
use card::ShipType;
use choice::Choice;

pub fn battle_blob() -> Card {
    Card {
        abilities: vec!(Choice::GainAttack(8)),
        card_type: CardType::Ship,
        cost: 6,
        faction: Faction::Blob,
        name: String::from("Battle Blob"),
        ship_type: ShipType::BattleBlob,
        ..Default::default()
    }
}

pub fn battle_pod() -> Card {
    Card {
        abilities: vec!(Choice::GainAttack(4)),
        card_type: CardType::Ship,
        cost: 2,
        faction: Faction::Blob,
        name: String::from("Battle Pod"),
        ship_type: ShipType::BattlePod,
        ..Default::default()
    }
}

pub fn blob_carrier() -> Card {
    Card {
        ally_abilities: vec!(Choice::AcquireFromTradeRow(0)),
        abilities: vec!(Choice::GainAttack(7)),
        card_type: CardType::Ship,
        cost: 6,
        faction: Faction::Blob,
        name: String::from("Blob Carrier"),
        ship_type: ShipType::BlobCarrier,
        ..Default::default()
    }
}

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

pub fn imperial_fighter() -> Card {
    Card {
        abilities: vec!(Choice::GainAttack(2), Choice::DiscardAttack(0)),
        ally_abilities: vec!(Choice::GainAttack(2)),
        card_type: CardType::Ship,
        cost: 1,
        faction: Faction::StarEmpire,
        name: String::from("Imperial Fighter"),
        ship_type: ShipType::ImperialFighter,
        ..Default::default()
    }
}

pub fn scout() -> Card {
    Card {
        abilities: vec!(Choice::GainTrade(1)),
        card_type: CardType::Ship,
        name: String::from("Scout"),
        ship_type: ShipType::Scout,
        ..Default::default()
    }
}

pub fn viper() -> Card {
    Card {
        abilities: vec!(Choice::GainAttack(1)),
        card_type: CardType::Ship,
        name: String::from("Viper"),
        ship_type: ShipType::Viper,
        ..Default::default()
    }
}

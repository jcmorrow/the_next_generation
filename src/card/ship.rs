use card::Card;
use card::CardType;
use card::Faction;
use card::ShipType;
use choice::Choice;

pub fn battle_blob() -> Card {
    Card {
        card_type: CardType::Ship,
        cost: 6,
        combat: 8,
        faction: Faction::Blob,
        name: String::from("Battle Blob"),
        ship_type: ShipType::BattleBlob,
        ..Default::default()
    }
}

pub fn battle_pod() -> Card {
    Card {
        card_type: CardType::Ship,
        combat: 4,
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
        card_type: CardType::Ship,
        combat: 7,
        cost: 6,
        faction: Faction::Blob,
        name: String::from("Blob Carrier"),
        ship_type: ShipType::BlobCarrier,
        ..Default::default()
    }
}

pub fn explorer() -> Card {
    Card {
        card_type: CardType::Ship,
        cost: 2,
        name: String::from("Explorer"),
        ship_type: ShipType::Explorer,
        trade: 2,
        scrappable: true,
        ..Default::default()
    }
}

pub fn imperial_fighter() -> Card {
    Card {
        abilities: vec!(Choice::DiscardAttack(0)),
        ally_abilities: vec!(Choice::GainAttack(2)),
        card_type: CardType::Ship,
        combat: 2,
        cost: 1,
        faction: Faction::StarEmpire,
        name: String::from("Imperial Fighter"),
        ship_type: ShipType::ImperialFighter,
        ..Default::default()
    }
}

pub fn scout() -> Card {
    Card {
        card_type: CardType::Ship,
        name: String::from("Scout"),
        trade: 1,
        ship_type: ShipType::Scout,
        ..Default::default()
    }
}

pub fn viper() -> Card {
    Card {
        card_type: CardType::Ship,
        combat: 1,
        name: String::from("Viper"),
        ship_type: ShipType::Viper,
        ..Default::default()
    }
}

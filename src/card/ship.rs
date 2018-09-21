use card::Card;
use card::CardType;
use card::Faction;
use card::ShipType;

pub fn battle_blob() -> Card {
    Card {
        card_type: CardType::Ship,
        cost: 6,
        combat: 8,
        faction: Faction::Blob,
        has_ally_ability: true,
        name: String::from("BattleBlob"),
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
        has_ally_ability: true,
        name: String::from("Battle Pod"),
        ship_type: ShipType::BattlePod,
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

pub fn overpowered_blob_carrier() -> Card {
    Card {
        card_type: CardType::Ship,
        combat: 7,
        cost: 6,
        faction: Faction::Blob,
        name: String::from("Overpowered Blob Carrier"),
        ship_type: ShipType::BlobCarrier,
        ..Default::default()
    }
}

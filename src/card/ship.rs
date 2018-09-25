use card::Card;
use card::CardType;
use card::Faction;
use card::ShipType;
use choice::Choice;
use effect::Effect;

pub fn battle_blob() -> Card {
    Card {
        // ally_abilities: vec!(Choice::Draw),
        card_type: CardType::Ship,
        cost: 6,
        effects: vec!(Effect::GainCombat(8)),
        faction: Faction::Blob,
        name: String::from("Battle Blob"),
        scrap_effects: vec!(Effect::GainCombat(4)),
        ship_type: ShipType::BattleBlob,
        ..Default::default()
    }
}

pub fn battle_pod() -> Card {
    Card {
        abilities: vec!(Choice::ScrapFromTradeRow(0)),
        ally_effects: vec!(Effect::GainCombat(2)),
        card_type: CardType::Ship,
        cost: 2,
        effects: vec!(Effect::GainCombat(4)),
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
        cost: 6,
        effects: vec!(Effect::GainCombat(7)),
        faction: Faction::Blob,
        name: String::from("Blob Carrier"),
        ship_type: ShipType::BlobCarrier,
        ..Default::default()
    }
}

pub fn blob_destroyer() -> Card {
    Card {
        ally_abilities: vec!(
                            Choice::AndOr(
                                Box::new(Choice::DestroyBase(0, 0)),
                                Box::new(Choice::ScrapFromTradeRow(0)),
                                false,
                                false
                            )
                        ),
        card_type: CardType::Ship,
        cost: 4,
        effects: vec!(Effect::GainCombat(6)),
        faction: Faction::Blob,
        name: String::from("Blob Destoyer"),
        ship_type: ShipType::BlobDestroyer,
        ..Default::default()
    }
}

pub fn cutter() -> Card {
    Card {
        ally_effects: vec!(Effect::GainCombat(4)),
        card_type: CardType::Ship,
        cost: 1,
        effects: vec!(Effect::GainTrade(2), Effect::GainAuthority(4)),
        faction: Faction::TradeFederation,
        name: String::from("Cutter"),
        ship_type: ShipType::Cutter,
        ..Default::default()
    }
}

pub fn explorer() -> Card {
    Card {
        card_type: CardType::Ship,
        cost: 2,
        effects: vec!(Effect::GainTrade(2)),
        name: String::from("Explorer"),
        scrap_effects: vec!(Effect::GainCombat(2)),
        ship_type: ShipType::Explorer,
        ..Default::default()
    }
}

pub fn imperial_fighter() -> Card {
    Card {
        ally_effects: vec!(Effect::GainCombat(2)),
        card_type: CardType::Ship,
        cost: 1,
        effects: vec!(Effect::GainCombat(2), Effect::DiscardAttack(0)),
        faction: Faction::StarEmpire,
        name: String::from("Imperial Fighter"),
        ship_type: ShipType::ImperialFighter,
        ..Default::default()
    }
}

pub fn scout() -> Card {
    Card {
        card_type: CardType::Ship,
        effects: vec!(Effect::GainTrade(1)),
        name: String::from("Scout"),
        ship_type: ShipType::Scout,
        ..Default::default()
    }
}

pub fn trade_bot() -> Card {
    Card {
        abilities: vec!(
            Choice::Or(
                Box::new(Choice::ScrapHand(0)),
                Box::new(Choice::ScrapDiscard(0)),
                true
            )),
        ally_effects: vec!(Effect::GainCombat(2)),
        card_type: CardType::Ship,
        cost: 1,
        effects: vec!(Effect::GainTrade(1)),
        faction: Faction::MachineCult,
        name: String::from("Trade Bot"),
        ship_type: ShipType::TradeBot,
        ..Default::default()
    }
}

pub fn viper() -> Card {
    Card {
        card_type: CardType::Ship,
        effects: vec!(Effect::GainCombat(1)),
        name: String::from("Viper"),
        ship_type: ShipType::Viper,
        ..Default::default()
    }
}

use card::Card;
use card::CardType;
use card::Faction;
use card::ShipType;
use choice::Choice;
use effect::Effect;

pub fn battlecruiser() -> Card {
    Card {
        ally_effects: vec!(Effect::DiscardAttack(0)),
        card_type: CardType::Ship,
        cost: 6,
        effects: vec!(Effect::GainCombat(5), Effect::Draw),
        faction: Faction::StarEmpire,
        name: String::from("Battlecruiser"),
        scrap_abilities: vec!(Choice::DestroyBase(0, 0)),
        scrap_effects: vec!(Effect::Draw),
        ship_type: ShipType::Battlecruiser,
        ..Default::default()
    }
}

pub fn battle_blob() -> Card {
    Card {
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

pub fn battle_mech() -> Card {
    Card {
        abilities: vec!(Choice::Or(
            Box::new(Choice::ScrapHand(0)),
            Box::new(Choice::ScrapDiscard(0)),
            true
        )),
        ally_effects: vec!(Effect::Draw),
        card_type: CardType::Ship,
        cost: 5,
        effects: vec!(Effect::GainCombat(4)),
        faction: Faction::MachineCult,
        name: String::from("Battle Mech"),
        ship_type: ShipType::BattleMech,
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

pub fn blob_fighter() -> Card {
    Card {
        ally_effects: vec!(Effect::Draw),
        card_type: CardType::Ship,
        cost: 1,
        effects: vec!(Effect::GainCombat(3)),
        faction: Faction::Blob,
        name: String::from("Blob Fighter"),
        ship_type: ShipType::BlobFighter,
        ..Default::default()
    }
}

pub fn corvette() -> Card {
    Card {
        ally_effects: vec!(Effect::GainCombat(2)),
        card_type: CardType::Ship,
        cost: 2,
        effects: vec!(Effect::GainCombat(1), Effect::Draw),
        faction: Faction::StarEmpire,
        name: String::from("Corvette"),
        ship_type: ShipType::Corvette,
        ..Default::default()
    }
}

pub fn dreadnaught() -> Card {
    Card {
        card_type: CardType::Ship,
        cost: 7,
        effects: vec!(Effect::GainCombat(7), Effect::Draw),
        faction: Faction::StarEmpire,
        name: String::from("Dreadnaught"),
        scrap_effects: vec!(Effect::GainCombat(5)),
        ship_type: ShipType::Dreadnaught,
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

pub fn embassy_yacht() -> Card {
    Card {
        card_type: CardType::Ship,
        cost: 3,
        effects: vec!(Effect::GainAuthority(3), Effect::GainTrade(2)),
        name: String::from("Embassy Yacht"),
        ship_type: ShipType::EmbassyYacht,
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

pub fn imperial_frigate() -> Card {
    Card {
        ally_effects: vec!(Effect::GainCombat(2)),
        card_type: CardType::Ship,
        cost: 3,
        effects: vec!(Effect::GainCombat(4), Effect::DiscardAttack(0)),
        faction: Faction::StarEmpire,
        name: String::from("Imperial Frigate"),
        scrap_effects: vec!(Effect::Draw),
        ship_type: ShipType::ImperialFrigate,
        ..Default::default()
    }
}

pub fn missle_bot() -> Card {
    Card {
        abilities: vec!(
            Choice::Or(
                Box::new(Choice::ScrapHand(0)),
                Box::new(Choice::ScrapDiscard(0)),
                true
            )),
        ally_effects: vec!(Effect::GainCombat(2)),
        card_type: CardType::Ship,
        cost: 2,
        effects: vec!(Effect::GainCombat(2)),
        faction: Faction::MachineCult,
        name: String::from("Missle Bot"),
        ship_type: ShipType::MissleBot,
        ..Default::default()
    }
}

pub fn missle_mech() -> Card {
    Card {
        abilities: vec!(Choice::DestroyBase(0, 0)),
        ally_effects: vec!(Effect::Draw),
        card_type: CardType::Ship,
        cost: 6,
        effects: vec!(Effect::GainCombat(6)),
        faction: Faction::MachineCult,
        name: String::from("Missle Mech"),
        ship_type: ShipType::MissleMech,
        ..Default::default()
    }
}

pub fn mothership() -> Card {
    Card {
        ally_effects: vec!(Effect::Draw),
        card_type: CardType::Ship,
        cost: 7,
        effects: vec!(Effect::GainCombat(6), Effect::Draw),
        faction: Faction::Blob,
        name: String::from("Mothership"),
        ship_type: ShipType::Mothership,
        ..Default::default()
    }
}

pub fn patrol_mech() -> Card {
    Card {
        abilities: vec!(
            Choice::Or(
                Box::new(Choice::GainTrade(3)),
                Box::new(Choice::GainCombat(5)),
                true
            )),
        ally_abilities: vec!(
            Choice::Or(
                Box::new(Choice::ScrapHand(0)),
                Box::new(Choice::ScrapDiscard(0)),
                true
            )
        ),
        card_type: CardType::Ship,
        cost: 4,
        faction: Faction::MachineCult,
        name: String::from("Patrol Mech"),
        ship_type: ShipType::PatrolMech,
        ..Default::default()
    }
}

pub fn ram() -> Card {
    Card {
        ally_effects: vec!(Effect::GainCombat(2)),
        card_type: CardType::Ship,
        cost: 3,
        effects: vec!(Effect::GainCombat(5)),
        faction: Faction::Blob,
        name: String::from("Ram"),
        scrap_effects: vec!(Effect::GainTrade(3)),
        ship_type: ShipType::Ram,
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

pub fn supply_bot() -> Card {
    Card {
        abilities: vec!(
            Choice::Or(
                Box::new(Choice::ScrapHand(0)),
                Box::new(Choice::ScrapDiscard(0)),
                true
            )
        ),
        ally_effects: vec!(Effect::GainCombat(2)),
        card_type: CardType::Ship,
        cost: 3,
        effects: vec!(Effect::GainTrade(2)),
        faction: Faction::MachineCult,
        name: String::from("Supply Bot"),
        ship_type: ShipType::SupplyBot,
        ..Default::default()
    }
}

pub fn survey_ship() -> Card {
    Card {
        ally_effects: vec!(Effect::GainCombat(2)),
        card_type: CardType::Ship,
        cost: 3,
        effects: vec!(Effect::GainTrade(1), Effect::Draw),
        faction: Faction::StarEmpire,
        name: String::from("Survey Ship"),
        scrap_effects: vec!(Effect::DiscardAttack(0)),
        ship_type: ShipType::SurveyShip,
        ..Default::default()
    }
}

// pub fn stealth_needle() -> Card {
//      TODO
// }

pub fn trade_pod() -> Card {
    Card {
        ally_effects: vec!(Effect::GainCombat(2)),
        card_type: CardType::Ship,
        cost: 2,
        effects: vec!(Effect::GainTrade(3)),
        faction: Faction::Blob,
        name: String::from("Trade Pod"),
        ship_type: ShipType::TradePod,
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

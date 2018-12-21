use card::Card;
use card::CardType;
use card::Faction;
use card::ShipType;
use choice::Ability;
use effect::Effect;

pub fn battlecruiser() -> Card {
    Card {
        ally_effects: vec!(Effect::DiscardAttack(0)),
        card_type: CardType::Ship,
        cost: 6,
        effects: vec!(Effect::GainCombat(5), Effect::Draw),
        faction: Faction::StarEmpire,
        name: String::from("Battlecruiser"),
        scrap_abilities: vec!(Ability::DestroyBase),
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
        abilities: vec!(Ability::Or(
            Box::new(Ability::ScrapHand),
            Box::new(Ability::ScrapDiscard),
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
        abilities: vec!(Ability::ScrapFromTradeRow),
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
        ally_abilities: vec!(Ability::AcquireFromTradeRow),
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
        ally_abilities: vec!(Ability::AndOr(
                                Box::new(Ability::DestroyBase),
                                Box::new(Ability::ScrapFromTradeRow),
                            )),
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
        cost: 2,
        effects: vec!(Effect::GainTrade(2), Effect::GainAuthority(4)),
        faction: Faction::TradeFederation,
        name: String::from("Cutter"),
        ship_type: ShipType::Cutter,
        ..Default::default()
    }
}

pub fn command_ship() -> Card {
    Card {
        ally_abilities: vec!(Ability::DestroyBase),
        card_type: CardType::Ship,
        cost: 8,
        effects: vec!(Effect::GainCombat(5), Effect::GainAuthority(4), Effect::Draw, Effect::Draw),
        faction: Faction::TradeFederation,
        name: String::from("Command Ship"),
        ship_type: ShipType::CommandShip,
        ..Default::default()
    }
}

pub fn embassy_yacht() -> Card {
    Card {
        card_type: CardType::Ship,
        cost: 3,
        effects: vec!(Effect::GainAuthority(3), Effect::GainTrade(2)),
        faction: Faction::TradeFederation,
        name: String::from("Embassy Yacht"),
        ship_type: ShipType::EmbassyYacht,
        ..Default::default()
    }
}

pub fn federation_shuttle() -> Card {
    Card {
        ally_effects: vec!(Effect::GainAuthority(4)),
        card_type: CardType::Ship,
        cost: 1,
        effects: vec!(Effect::GainTrade(2)),
        faction: Faction::TradeFederation,
        name: String::from("Federation Shuttle"),
        ship_type: ShipType::FederationShuttle,
        ..Default::default()
    }
}

pub fn flagship() -> Card {
    Card {
        ally_effects: vec!(Effect::GainAuthority(5)),
        card_type: CardType::Ship,
        cost: 6,
        effects: vec!(Effect::GainCombat(5), Effect::Draw),
        faction: Faction::TradeFederation,
        name: String::from("Flagship"),
        ship_type: ShipType::Flagship,
        ..Default::default()
    }
}

pub fn freighter() -> Card {
    Card {
        ally_abilities: vec!(Ability::BuyTopDeck),
        card_type: CardType::Ship,
        cost: 4,
        effects: vec!(Effect::GainTrade(4)),
        faction: Faction::TradeFederation,
        name: String::from("Freighter"),
        ship_type: ShipType::Freighter,
        ..Default::default()
    }
}

pub fn trade_escort() -> Card {
    Card {
        ally_effects: vec!(Effect::Draw),
        card_type: CardType::Ship,
        cost: 5,
        effects: vec!(Effect::GainAuthority(4), Effect::GainCombat(4)),
        faction: Faction::TradeFederation,
        name: String::from("Trade Escort"),
        ship_type: ShipType::TradeEscort,
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

pub fn missile_bot() -> Card {
    Card {
        abilities: vec!(
            Ability::Or(
                Box::new(Ability::ScrapHand),
                Box::new(Ability::ScrapDiscard),
            )),
        ally_effects: vec!(Effect::GainCombat(2)),
        card_type: CardType::Ship,
        cost: 2,
        effects: vec!(Effect::GainCombat(2)),
        faction: Faction::MachineCult,
        name: String::from("Missile Bot"),
        ship_type: ShipType::MissileBot,
        ..Default::default()
    }
}

pub fn missile_mech() -> Card {
    Card {
        abilities: vec!(Ability::DestroyBase),
        ally_effects: vec!(Effect::Draw),
        card_type: CardType::Ship,
        cost: 6,
        effects: vec!(Effect::GainCombat(6)),
        faction: Faction::MachineCult,
        name: String::from("Missile Mech"),
        ship_type: ShipType::MissileMech,
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
            Ability::Or(
                Box::new(Ability::GainTrade(3)),
                Box::new(Ability::GainCombat(5)),
            )),
        ally_abilities: vec!(
            Ability::Or(
                Box::new(Ability::ScrapHand),
                Box::new(Ability::ScrapDiscard),
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
        cost: 1,
        card_type: CardType::Ship,
        effects: vec!(Effect::GainTrade(1)),
        name: String::from("Scout"),
        ship_type: ShipType::Scout,
        ..Default::default()
    }
}

pub fn stealth_needle() -> Card {
    Card {
        abilities: vec!(Ability::CopyShip),
        card_type: CardType::Ship,
        cost: 4,
        faction: Faction::MachineCult,
        name: String::from("Stealth Needle"),
        ship_type: ShipType::StealthNeedle,
        ..Default::default()
    }
}

pub fn supply_bot() -> Card {
    Card {
        abilities: vec!(
            Ability::Or(
                Box::new(Ability::ScrapHand),
                Box::new(Ability::ScrapDiscard),
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
            Ability::Or(
                Box::new(Ability::ScrapHand),
                Box::new(Ability::ScrapDiscard),
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
        cost: 1,
        card_type: CardType::Ship,
        effects: vec!(Effect::GainCombat(1)),
        name: String::from("Viper"),
        ship_type: ShipType::Viper,
        ..Default::default()
    }
}

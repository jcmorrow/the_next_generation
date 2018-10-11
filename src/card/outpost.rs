use card::Card;
use card::CardType;
use card::Faction;
use card::OutpostType;
use choice::Choice;
use effect::Effect;

pub fn battle_station() -> Card {
    Card {
        card_type: CardType::Outpost,
        cost: 3,
        faction: Faction::MachineCult,
        health: 5,
        name: String::from("Battle Station"),
        outpost_type: OutpostType::BattleStation,
        scrap_effects: vec!(Effect::GainCombat(5)),
        ..Default::default()
    }
}

// pub fn brain_world() -> Card {
//      TODO
// }

pub fn junkyard() -> Card {
    Card {
        abilities: vec!(Choice::Or(
            Box::new(Choice::ScrapHand(0)),
            Box::new(Choice::ScrapDiscard(0)),
            true
        )),
        card_type: CardType::Outpost,
        cost: 6,
        faction: Faction::MachineCult,
        health: 5,
        name: String::from("Junkyard"),
        outpost_type: OutpostType::Junkyard,
        ..Default::default()
    }
}

pub fn mech_world() -> Card {
    Card {
        card_type: CardType::Outpost,
        cost: 5,
        faction: Faction::MachineCult,
        health: 6,
        name: String::from("Mech World"),
        outpost_type: OutpostType::MechWorld,
        ..Default::default()
    }
}

// pub fn machine_base() -> Card {
//      TODO
// }

pub fn royal_redoubt() -> Card {
    Card {
        ally_effects: vec!(Effect::DiscardAttack(0)),
        card_type: CardType::Outpost,
        cost: 6,
        effects: vec!(Effect::GainCombat(3)),
        faction: Faction::StarEmpire,
        health: 6,
        name: String::from("Royal Redoubt"),
        outpost_type: OutpostType::RoyalRedoubt,
        ..Default::default()
    }
}

pub fn space_station() -> Card {
    Card {
        ally_effects: vec!(Effect::GainCombat(2)),
        card_type: CardType::Outpost,
        cost: 4,
        effects: vec!(Effect::GainCombat(2)),
        faction: Faction::StarEmpire,
        health: 4,
        name: String::from("Space Station"),
        outpost_type: OutpostType::SpaceStation,
        scrap_effects: vec!(Effect::GainTrade(4)),
        ..Default::default()
    }
}

pub fn trading_post() -> Card {
    Card {
        card_type: CardType::Outpost,
        cost: 3,
        faction: Faction::TradeFederation,
        abilities: vec!(
            Choice::Or(
                Box::new(Choice::GainAuthority(1)),
                Box::new(Choice::GainTrade(1)),
                true
            )
        ),
        health: 4,
        name: String::from("Trading Post"),
        outpost_type: OutpostType::TradingPost,
        ..Default::default()
    }
}

pub fn war_world() -> Card {
    Card {
        ally_effects: vec!(Effect::GainCombat(4)),
        card_type: CardType::Outpost,
        cost: 5,
        effects: vec!(Effect::GainCombat(3)),
        faction: Faction::StarEmpire,
        health: 4,
        name: String::from("War World"),
        outpost_type: OutpostType::WarWorld,
        ..Default::default()
    }
}

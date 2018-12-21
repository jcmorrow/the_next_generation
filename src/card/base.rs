use card::Card;
use card::CardType;
use card::Faction;
use card::BaseType;
use choice::Ability;
use effect::Effect;

pub fn barter_world() -> Card {
    Card {
        abilities: vec!(
            Ability::Or(
                Box::new(Ability::GainTrade(2)),
                Box::new(Ability::GainAuthority(2)),
            )
        ),
        base_type: BaseType::BarterWorld,
        card_type: CardType::Base,
        cost: 4,
        faction: Faction::TradeFederation,
        health: 5,
        name: String::from("Barter World"),
        scrap_effects: vec!(Effect::GainCombat(5)),
        ..Default::default()
    }
}

pub fn blob_wheel() -> Card {
    Card {
        base_type: BaseType::BlobWheel,
        card_type: CardType::Base,
        cost: 3,
        faction: Faction::Blob,
        health: 5,
        name: String::from("Blob Wheel"),
        scrap_effects: vec!(Effect::GainTrade(3)),
        ..Default::default()
    }
}

pub fn blob_world() -> Card {
    Card {
        abilities: vec!(Ability::Or(
            Box::new(Ability::GainCombat(5)),
            Box::new(Ability::BlobDraw),
        )),
        base_type: BaseType::BlobWorld,
        card_type: CardType::Base,
        cost: 8,
        faction: Faction::Blob,
        health: 7,
        name: String::from("Blob World"),
        ..Default::default()
    }
}

pub fn central_office() -> Card {
    Card {
        abilities: vec!(Ability::BuyTopDeck),
        ally_effects: vec!(Effect::Draw),
        base_type: BaseType::CentralOffice,
        card_type: CardType::Base,
        cost: 7,
        effects: vec!(Effect::GainTrade(2)),
        faction: Faction::TradeFederation,
        health: 6,
        name: String::from("Central Office"),
        scrap_effects: vec!(Effect::GainCombat(5)),
        ..Default::default()
    }
}


pub fn fleet_hq() -> Card {
    Card {
        base_type: BaseType::FleetHQ,
        card_type: CardType::Base,
        cost: 8,
        faction: Faction::StarEmpire,
        health: 8,
        name: String::from("Fleet HQ"),
        ..Default::default()
    }
}

pub fn the_hive() -> Card {
    Card {
        ally_effects: vec!(Effect::Draw),
        base_type: BaseType::TheHive,
        card_type: CardType::Base,
        cost: 5,
        faction: Faction::Blob,
        health: 5,
        name: String::from("The Hive"),
        ..Default::default()
    }
}

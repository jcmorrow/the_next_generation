use card::Card;
use card::CardType;
use card::Faction;
use card::BaseType;
use choice::Choice;
use effect::Effect;

pub fn barter_world() -> Card {
    Card {
        abilities: vec!(
            Choice::Or(
                Box::new(Choice::GainTrade(2)),
                Box::new(Choice::GainAuthority(2)),
                true
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
        abilities: vec!(Choice::Or(
            Box::new(Choice::GainCombat(5)),
            Box::new(Choice::BlobDraw(0)),
            true
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
        abilities: vec!(Choice::BuyTopDeck(0)),
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

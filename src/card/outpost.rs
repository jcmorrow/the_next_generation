use card::Card;
use card::CardType;
use card::Faction;
use card::OutpostType;
use choice::Choice;

pub fn battle_station() -> Card {
    Card {
        card_type: CardType::Outpost,
        cost: 3,
        faction: Faction::MachineCult,
        health: 5,
        name: String::from("Battle Station"),
        outpost_type: OutpostType::BattleStation,
        scrap_abilities: vec!(Choice::GainAttack(5)),
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

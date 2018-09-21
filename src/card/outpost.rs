use card::Card;
use card::CardType;
use card::Faction;
use card::OutpostType;

pub fn battle_station() -> Card {
    Card {
        card_type: CardType::Outpost,
        cost: 3,
        faction: Faction::MachineCult,
        health: 5,
        name: String::from("Battle Station"),
        outpost_type: OutpostType::BattleStation,
        scrappable: true,
        ..Default::default()
    }
}

use card;
use card::Card;
use card::Faction;

#[derive(Clone)]
pub enum Type {
    BattleStation,
    NoType,
}

impl Default for Type {
    fn default() ->  Type { Type::NoType }
}

pub fn battle_station() -> Card {
    Card {
        card_type: card::Type::Outpost,
        cost: 3,
        faction: Faction::MachineCult,
        health: 5,
        name: String::from("Battle Station"),
        outpost_type: Type::BattleStation,
        scrappable: true,
        ..Default::default()
    }
}

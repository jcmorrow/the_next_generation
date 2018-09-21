use card;
use card::Card;
use card::Faction;

#[derive(Clone)]
#[derive(Debug)]
pub enum Type {
    TheHive,
    NoType,
}

impl Default for Type {
    fn default() ->  Type { Type::NoType }
}

pub fn the_hive() -> Card {
    Card {
        card_type: card::Type::Base,
        cost: 5,
        faction: Faction::Blob,
        has_ally_ability: true,
        health: 5,
        name: String::from("The Hive"),
        base_type: Type::TheHive,
        ..Default::default()
    }
}

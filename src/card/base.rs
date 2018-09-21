use card::Card;
use card::CardType;
use card::Faction;
use card::BaseType;

pub fn the_hive() -> Card {
    Card {
        card_type: CardType::Base,
        cost: 5,
        faction: Faction::Blob,
        has_ally_ability: true,
        health: 5,
        name: String::from("The Hive"),
        base_type: BaseType::TheHive,
        ..Default::default()
    }
}

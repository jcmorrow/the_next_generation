use card;
use card::Card;
use card::Faction;

#[derive(Clone)]
#[derive(Debug)]
pub enum Type {
    BattleBlob,
    BattlePod,
    Explorer,
    NoType,
    Scout,
    SurveyShip,
    Viper,
}

impl Default for Type {
    fn default() ->  Type { Type::NoType }
}


pub fn battle_blob() -> Card {
    Card {
        card_type: card::Type::Ship,
        cost: 6,
        combat: 8,
        faction: Faction::Blob,
        has_ally_ability: true,
        name: String::from("BattleBlob"),
        ship_type: Type::BattleBlob,
        ..Default::default()
    }
}

pub fn battle_pod() -> Card {
    Card {
        card_type: card::Type::Ship,
        combat: 4,
        cost: 2,
        faction: Faction::Blob,
        has_ally_ability: true,
        name: String::from("Battle Pod"),
        ship_type: Type::BattlePod,
        ..Default::default()
    }
}

pub fn explorer() -> Card {
    Card {
        card_type: card::Type::Ship,
        cost: 2,
        name: String::from("Explorer"),
        ship_type: Type::Explorer,
        trade: 2,
        scrappable: true,
        ..Default::default()
    }
}

pub fn scout() -> Card {
    Card {
        card_type: card::Type::Ship,
        name: String::from("Scout"),
        trade: 1,
        ship_type: Type::Scout,
        ..Default::default()
    }
}

pub fn survey_ship() -> Card {
    Card {
        card_type: card::Type::Ship,
        cost: 3,
        faction: Faction::StarEmpire,
        name: String::from("Survey Ship"),
        trade: 1,
        ship_type: Type::SurveyShip,
        scrappable: true,
        ..Default::default()
    }
}

pub fn viper() -> Card {
    Card {
        card_type: card::Type::Ship,
        combat: 1,
        name: String::from("Viper"),
        ship_type: Type::Viper,
        ..Default::default()
    }
}

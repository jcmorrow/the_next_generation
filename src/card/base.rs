use card::Card;
use card::CardType;
use card::Faction;

impl Card {
    pub fn barter_world() -> Card {
        Card{
            name: String::from("Barter World"),
            cost: 4,
            combat: 0,
            trade: 0,
            card_type: CardType::Base,
            faction: Faction::TradeFederation
        }
    }
}

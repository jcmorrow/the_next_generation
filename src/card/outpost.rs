use card::Card;
use card::CardType;
use card::Faction;

impl Card {
    pub fn war_world() -> Card {
        Card{
            name: String::from("War World"),
            cost: 5,
            combat: 3,
            trade: 0,
            card_type: CardType::Outpost,
            faction: Faction::StarEmpire
        }
    }
}

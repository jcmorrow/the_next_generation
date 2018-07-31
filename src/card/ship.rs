use card::Card;
use card::CardType;
use card::Faction;

impl Card {
    pub fn scout() -> Card {
        Card{
            name: String::from("Scout"),
            cost: 0,
            combat: 0,
            trade: 1,
            card_type: CardType::Ship,
            faction: Faction::Unaligned
        }
    }

    pub fn viper() -> Card {
        Card{
            name: String::from("Viper"),
            cost: 0,
            combat: 1,
            trade: 0,
            card_type: CardType::Ship,
            faction: Faction::Unaligned
        }
    }

    pub fn trade_bot() -> Card {
        Card{
            name: String::from("Trade Bot"),
            cost: 1,
            combat: 0,
            trade: 1,
            card_type: CardType::Ship,
            faction: Faction::MachineCult
        }
    }

    pub fn blob_fighter() -> Card {
        Card{
            name: String::from("Blob Fighter"),
            cost: 1,
            combat: 3,
            trade: 0,
            card_type: CardType::Ship,
            faction: Faction::Blob
        }
    }
}

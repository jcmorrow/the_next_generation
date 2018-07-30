
mod card;

use card::Card;
use card::Faction;
use card::CardType;

fn main() {
    let federation_shuttle = Card {
        card_type: CardType::Ship,
        faction: Faction::TradeFederation,
        name: String::from("Federation Shuttle")
    };

    println!("{:?}", federation_shuttle);
}


mod card;
mod player;

use card::Card;
use card::Faction;
use card::CardType;

use player::Player;

fn main() {
    let federation_shuttle = Card {
        card_type: CardType::Ship,
        faction: Faction::TradeFederation,
        name: String::from("Federation Shuttle")
    };

    let player_1 = Player::new("Cameron");
    let player_2 = Player::new("Josh");

    println!("{:?}", federation_shuttle);
    println!("{:?}", player_1);
    println!("{:?}", player_2);
}

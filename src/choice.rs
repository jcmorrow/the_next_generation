use std::fmt;

use card::Card;
use trade_row::TradeRow;
use player::Player;

#[derive(Debug)]
#[derive(Clone)]
pub enum Choice {
    AcquireFromTradeRow(usize),
    Buy(usize),
    DiscardAttack(usize),
    DiscardCard(usize),
    EndTurn,
    GainAttack(i32),
    Play(usize),
    Scrap(usize),
    Attack(usize),
}

impl Choice {
    pub fn choose(self,
                  player: &mut Player,
                  opponents: &mut [&mut Player],
                  trade_row: &mut TradeRow) {
        match self {
            Choice::Play(i) => {
                let mut card = player.hand.remove(i);
                println!("{} plays {}", player.name, card.name);
                card.run(player, opponents, trade_row);
                player.in_play.push(card);
            },
            Choice::DiscardCard(i) => {
                let mut card = player.hand.remove(i);
                println!("{} discards {}", player.name, card.name);
                player.discard.push(card);
            },
            Choice::AcquireFromTradeRow(i) => {
                let card = trade_row.buy(i);
                println!("{} acquires {} to the top of the deck", player.name, card.name);
                player.deck.insert(0, card);
            },
            Choice::Buy(i) => {
                let card = trade_row.buy(i);
                println!("{} buys {}", player.name, card.name);
                player.trade -= card.cost;
                player.discard.push(card);
            },
            Choice::Attack(i) => {
                let combat = player.combat;
                println!("{} attacks {} for {}",
                         player.name,
                         opponents[i].name,
                         combat);
                opponents[i].authority -= combat;
                player.combat = 0;
            },
            Choice::DiscardAttack(opponent_index) => {
                println!("{} makes {} discard!", player.name, opponents[opponent_index].name);
                opponents[opponent_index].turn_start_choices.push(
                    Choice::DiscardCard(0))
            },
            Choice::GainAttack(n) => {
                player.combat += n;
            },
            _ => (),
        }
    }
}

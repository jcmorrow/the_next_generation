use std::fmt;

use event::Event;
use card::Card;
use trade_row::TradeRow;
use player::Player;

#[derive(Debug)]
pub enum Choice {
    AcquireFromTradeRow(usize),
    GainCombat(i32),
    GainTrade(i32),
    Buy(usize),
    EndTurn,
    Play(usize),
    Scrap(Card),
    Attack(Player),
}

impl Choice {
    pub fn choose<'a>(&'a self,
                  player: &'a mut Player,
                  opponents: &[&mut Player],
                  trade_row: &mut TradeRow) -> Event<'a> {
        match self {
            Choice::Play(i) => {
                let card = player.hand.remove(*i);
                println!("{} plays {}", player.name, card.name);
                card.run(player, opponents, trade_row);
                player.in_play.push(card);
            },
            Choice::AcquireFromTradeRow(i) => {
                let card = trade_row.buy(*i);
                println!("{} acquires {} to the top of the deck", player.name, card.name);
                player.deck.insert(0, card);
            },
            Choice::Buy(i) => {
                let card = trade_row.buy(*i);
                println!("{} buys {}", player.name, card.name);
                player.trade -= card.cost;
                player.discard.push(card);
            },
            Choice::GainTrade(n) => {
                println!("{} gains {} trade", player.name, n);
                player.trade += n;
            },
            Choice::GainCombat(n) => {
                println!("{} gains {} combat", player.name, n);
                player.combat += n;
            },
            _ => (),
        };
        Event::new(&player, &self, "log")
    }
}

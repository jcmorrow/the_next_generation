use std::fmt;

use card::Card;
use trade_row::TradeRow;
use player::Player;

#[derive(Debug)]
pub enum Choice<'a> {
    AcquireFromTradeRow(&'a Card),
    GainCombat(i32),
    GainTrade(i32),
    Buy(Card),
    EndTurn,
    Play(Card),
    Scrap(Card),
    Attack(Player<'a>),
}

impl<'a> Choice<'a> {
    pub fn choose(self,
                  player: &mut Player<'a>,
                  opponents: &[&mut Player],
                  trade_row: &'a mut TradeRow) {
        match self {
            Choice::Play(c) => {
                c.run(player, opponents, trade_row);
                player.in_play.push(c);
            },
            _ => (),
        }
    }
}

impl<'a> fmt::Display for Choice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Choice::Buy(c) => write!(f, "buys {}", c),
            Choice::EndTurn => write!(f, "ends turns"),
            Choice::Play(c) => write!(f, "plays {}", c),
            Choice::Scrap(c) => write!(f, "scraps {}", c),
            Choice::Attack(p) => write!(f, "attacks {}", p),
            Choice::GainCombat(p) => write!(f, "gains {} combat", p),
            Choice::GainTrade(p) => write!(f, "gains {} trade", p),
            Choice::AcquireFromTradeRow(p) => write!(f, "acquires {} from trade", p)
        }
    }
}

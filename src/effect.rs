use trade_row::TradeRow;
use player::Player;

#[derive(Debug)]
#[derive(Clone)]
pub enum Effect {
    GainCombat(i32),
    GainTrade(i32)
}

impl Effect {
    pub fn process(self,
                   player: &mut Player,
                   opponents: &mut [&mut Player],
                   trade_row: &TradeRow) {

        match self {
            Effect::GainCombat(n) => {
                println!("{} gains {} combat", player.name, n);
                player.combat += n;
            },
            Effect::GainTrade(n) => {
                println!("{} gains {} trade", player.name, n);
                player.trade += n;
            }
        }
    }
}

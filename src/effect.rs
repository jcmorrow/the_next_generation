use choice::Choice;
use trade_row::TradeRow;
use player::Player;

#[derive(Debug)]
#[derive(Clone)]
pub enum Effect {
    DiscardAttack(usize),
    GainAuthority(i32),
    GainCombat(i32),
    GainTrade(i32),
    ScrapSelf(usize),
    Empty
}

impl Effect {
    pub fn process(self,
                   player: &mut Player,
                   opponents: &mut [&mut Player],
                   trade_row: &TradeRow) {

        match self {
            Effect::GainAuthority(n) => {
                println!("{} gains {} authority", player.name, n);
                player.authority += n;
            },
            Effect::GainCombat(n) => {
                println!("{} gains {} combat", player.name, n);
                player.combat += n;
            },
            Effect::GainTrade(n) => {
                println!("{} gains {} trade", player.name, n);
                player.trade += n;
            },
            Effect::DiscardAttack(opponent_index) => {
                println!("{} makes {} discard!", player.name, opponents[opponent_index].name);
                opponents[opponent_index].turn_start_choices.push(
                    Choice::DiscardCard(0))
            },
            _ => ()
        }
    }
}

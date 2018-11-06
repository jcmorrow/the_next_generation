use choice::Ability;
use player::Player;
use trade_row::TradeRow;

#[derive(Debug)]
#[derive(Clone)]
pub enum Effect {
    DiscardAttack(usize),
    Draw,
    GainAuthority(i32),
    GainCombat(i32),
    GainTrade(i32),
    ScrapHand(usize),
    Empty
}

impl Effect {
    pub fn process(&self,
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
                println!("{} makes {} discard!", player.name, opponents[*opponent_index].name);
                opponents[*opponent_index].mandatory_abilities.push(Ability::DiscardCard)
            },
            Effect::Draw => {
                println!("{} draws a card", player.name);
                player.draw();
            },
            Effect::ScrapHand(i) => {
                let card = player.hand.remove(*i);
                player.scrapped.push(card);
            },
            _ => ()
        }
    }
}

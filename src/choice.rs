use trade_row::TradeRow;
use player::Player;

#[derive(Debug)]
#[derive(Clone)]
pub enum Choice {
    AcquireFromTradeRow(usize),
    Attack(usize),
    Buy(usize),
    Decline,
    DestroyBase(usize, usize),
    DiscardAttack(usize),
    DiscardCard(usize),
    AndOr(Box<Choice>, Box<Choice>, bool, bool),
    EndTurn,
    GainAttack(i32),
    GainAuthority(i32),
    GainTrade(i32),
    Or(Box<Choice>, Box<Choice>, bool),
    Play(usize),
    ScrapDiscard(usize),
    ScrapHand(usize),
    Scrap(usize),
    ScrapFromTradeRow(usize),
    ScrapSelf(usize),
}

pub struct Event<'a> {
    player: &'a Player,
    // card: &'a Card,
    summary: String,
}

impl<'a> Event<'a> {
    pub fn new<'b>(summary: &str,
                   // _card: &'b Card,
                   player: &'b Player) -> Event<'b> {
        Event {
            summary: summary.to_string(),
            player: player,
        }
    }
}

impl Choice {
    pub fn choose<'a>(self,
                  player: &mut Player,
                  mut opponents: &mut [&mut Player],
                  trade_row: &mut TradeRow,
                  current_player_ref: &'a Player) -> Event<'a> {
        match self {
            Choice::Play(i) => {
                let mut card = player.hand.remove(i);
                println!("{} plays {}", player.name, card.name);
                card.run(player, &mut opponents, trade_row);
                player.in_play.push(card);
            },
            Choice::DestroyBase(opponent, base) => {
                let chosen_opponent = &mut opponents[opponent];
                let chosen_base = chosen_opponent.in_play.remove(base);
                println!("{} destroys {}'s {}",
                         player.name,
                         chosen_opponent.name,
                         chosen_base);
                chosen_opponent.discard.push(chosen_base);
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
                println!("{} gains {} attack", player.name, n);
                player.combat += n;
            },
            Choice::GainAuthority(n) => {
                println!("{} gains {} authority", player.name, n);
                player.authority += n;
            },
            Choice::GainTrade(n) => {
                println!("{} gains {} trade", player.name, n);
                player.trade += n;
            },
            Choice::Or(a, b, first) => {
                let choice = match first {
                    true => { *a },
                    false => { *b }
                };
                println!("{} chooses {:?}", player.name, choice);
                player.choices.push(choice);
            },
            Choice::ScrapDiscard(i) => {
                let card = player.discard.remove(i);
                println!("{} scraps {} from discard", player.name, card.name);
                player.scrapped.push(card);
            },
            Choice::ScrapHand(i) => {
                let card = player.hand.remove(i);
                println!("{} scraps {} from hand", player.name, card.name);
                player.scrapped.push(card);
            },
            Choice::AndOr(a, b, first, second) => {
                if first {
                    player.choices.push(*a);
                }
                if second {
                    player.choices.push(*b);
                }
            },
            Choice::ScrapSelf(i) => {
                let card = player.in_play.remove(i);
                println!("{}'s {} scraps itself", player.name, card.name);
                player.choices.extend(card.scrap_abilities.clone());
                player.scrapped.push(card);
            },
            _ => (),
        }

        Event::new("Play", current_player_ref)
    }
}

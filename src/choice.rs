use card::Faction;
use card::ShipType;
use effect::Effect;
use trade_row::TradeRow;
use player::Player;

#[derive(Debug)]
#[derive(Clone)]
pub enum Choice {
    AcquireFromTradeRow(usize),
    AndOr(Box<Choice>, Box<Choice>, bool, bool),
    Attack(usize),
    BlobDraw(usize),
    Buy(usize),
    BuyTopDeck(usize),
    CopyShip(usize),
    Decline,
    DestroyBase(usize, usize),
    DiscardCard(usize),
    DiscardCardDraw(usize),
    DiscardDraw(i32),
    DrawScrap(usize),
    EndTurn,
    GainAuthority(i32),
    GainCombat(i32),
    GainTrade(i32),
    Or(Box<Choice>, Box<Choice>, bool),
    Play(usize),
    ScrapDiscard(usize),
    ScrapDiscardDraw(usize),
    ScrapDraw(i32),
    ScrapFromTradeRow(usize),
    ScrapHand(usize),
    ScrapHandDraw(usize),
    ScrapSelf(usize),
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
                if card.faction == Faction::Blob {
                    player.blobs_played_this_turn += 1;
                }
                card.run(player, opponents, trade_row);
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
            Choice::DiscardDraw(n) => {
                for _ in 0..n {
                    player.choices.push(Choice::DiscardCardDraw(0));
                }
            },
            Choice::DiscardCardDraw(i) => {
                let mut card = player.hand.remove(i);
                println!("{} discards {} and draws a card", player.name, card.name);
                player.discard.push(card);
                player.effects.push(Effect::Draw);
            },
            Choice::AcquireFromTradeRow(i) => {
                let card = trade_row.get_card(i);
                println!("{} acquires {} to the top of the deck", player.name, card.name);
                player.deck.insert(0, card);
            },
            Choice::ScrapFromTradeRow(i) => {
                let card = trade_row.get_card(i);
                println!("{} has been scrapped from the Trade Row", card.name);
                trade_row.scrapped.push(card);
            }
            Choice::Buy(i) => {
                let card = trade_row.get_card(i);
                println!("{} buys {}", player.name, card.name);
                player.trade -= card.cost;
                player.discard.push(card);
            },
            Choice::BuyTopDeck(i) => {
                let card = trade_row.get_card(i);
                println!("{} buys {} and places it on top of deck", player.name, card.name);
                player.trade -= card.cost;
                player.deck.insert(0, card);
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
            Choice::Or(a, b, first) => {
                let choice = match first {
                    true => { *a },
                    false => { *b }
                };
                println!("{} chooses {:?}", player.name, choice);
                player.choices.push(choice);
            },
            Choice::ScrapDiscard(i) => {
                let card = &player.discard[i];
                println!("{} scraps {} from discard", player.name, card.name);
                player.effects.push(Effect::ScrapDiscard(i));
            },
            Choice::ScrapHand(i) => {
                let card = &player.hand[i];
                println!("{} scraps {} from hand", player.name, card.name);
                player.effects.push(Effect::ScrapHand(i));
            },
            Choice::ScrapDiscardDraw(i) => {
                let card = &player.discard[i];
                println!("{} scraps {} from discard and draws a card", player.name, card.name);
                player.effects.push(Effect::ScrapDiscard(i));
                player.effects.push(Effect::Draw);
            },
            Choice::ScrapHandDraw(i) => {
                let card = &player.hand[i];
                println!("{} scraps {} from hand and draws a card", player.name, card.name);
                player.effects.push(Effect::ScrapHand(i));
                player.effects.push(Effect::Draw);
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
                let card = &player.in_play[i];
                println!("{} chooses to scrap {}", player.name, card.name);
                player.choices.extend(card.scrap_abilities.clone());
                player.effects.extend(card.scrap_effects.clone());
                player.effects.push(Effect::ScrapSelf(i));
            },
            Choice::BlobDraw(n) => {
                println!("{} draws {} cards from Blob World", player.name, n);
                for _ in 0..n {
                    player.effects.push(Effect::Draw);
                }
            },
            // Scrap then draw. Used by Brain World.
            Choice::ScrapDraw(n) => {
                for _ in 0..n {
                    player.choices.push(
                        Choice::Or(
                            Box::new(Choice::ScrapDiscardDraw(0)),
                            Box::new(Choice::ScrapHandDraw(0)),
                            true
                        )
                    );
                }
            },
            Choice::CopyShip(i) => {
                let mut ship_to_copy = player.in_play[i].clone();
                println!("{}'s Stealth Needle copies {}", player.name, ship_to_copy.name);
                if ship_to_copy.faction == Faction::Blob {
                    player.blobs_played_this_turn += 1;
                }
                if ship_to_copy.scrap_abilities.len() > 0 || ship_to_copy.scrap_effects.len() > 0 {
                    player.perrenial_choices.push(Choice::ScrapSelf(i));
                }
                ship_to_copy.run(player, opponents, trade_row);
            },
            // Draw then scrap. Used by Machine Base.
            Choice::DrawScrap(i) => {
                player.effects.push(Effect::Draw);
                player.turn_start_choices.push(Choice::ScrapHand(i));
            },
            Choice::GainAuthority(n) => {
                player.effects.push(Effect::GainAuthority(n));
            },
            Choice::GainCombat(n) => {
                player.effects.push(Effect::GainCombat(n));
            },
            Choice::GainTrade(n) => {
                player.effects.push(Effect::GainTrade(n));
            },
            _ => (),
        }
        player.process_effects(&trade_row, opponents);
    }
}

use card::Faction;
use card::CardType;
use card::ShipType;
use effect::Effect;
use trade_row::TradeRow;
use player::Player;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Choice {
    AcquireFromTradeRow(usize),
    AndOr(Box<Choice>, Box<Choice>, bool, bool),
    Attack(usize),
    BlobDraw(usize),
    Buy(usize),
    BuyTopDeck(usize),
    CopyShip(usize),
    DestroyBase(usize, usize),
    DiscardCard(usize),
    DiscardCardDraw(usize),
    DiscardThenDraw(usize),
    DrawThenScrap,
    EndTurn,
    GainAuthority(i32),
    GainCombat(i32),
    GainTrade(i32),
    Or(Box<Choice>, Box<Choice>, bool),
    Play(usize),
    ScrapDiscard(usize),
    ScrapDiscardDraw(usize),
    ScrapDraw(usize),
    ScrapFromTradeRow(usize),
    ScrapHand(usize),
    ScrapHandDraw(usize),
    ScrapSelf(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ability {
    AndOr(Box<Ability>, Box<Ability>),
    Or(Box<Ability>, Box<Ability>),

    AcquireFromTradeRow,
    Attack,
    BlobDraw,
    Buy,
    BuyTopDeck,
    CopyShip,
    DestroyBase,
    DiscardCard,
    DiscardCardDraw,
    DiscardThenDraw,
    DrawThenScrap,
    EndTurn,
    GainAuthority(i32),
    GainCombat(i32),
    GainTrade(i32),
    Play,
    ScrapDiscard,
    ScrapDiscardDraw,
    ScrapDraw,
    ScrapFromTradeRow,
    ScrapHand,
    ScrapHandDraw,
    ScrapSelf,
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
                player.lose_ability(Ability::DestroyBase);
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
                player.mandatory_abilities.pop().unwrap();
            },
            Choice::DiscardThenDraw(n) => {
                for _ in 0..n {
                    let count = player.abilities.entry(
                        Ability::DiscardCardDraw
                    ).or_insert(0);
                    *count += 1;
                }
                player.lose_ability(Ability::DiscardThenDraw);
            },
            Choice::DiscardCardDraw(i) => {
                let mut card = player.hand.remove(i);
                println!("{} discards {} and draws a card", player.name, card.name);
                player.discard.push(card);
                player.effects.push(Effect::Draw);
                player.lose_ability(Ability::DiscardCardDraw);
            },
            Choice::AcquireFromTradeRow(i) => {
                let card = trade_row.get_card(i);
                println!("{} acquires {} to the top of the deck", player.name, card.name);
                player.deck.insert(0, card);
                player.lose_ability(Ability::AcquireFromTradeRow);
            },
            Choice::ScrapFromTradeRow(i) => {
                let card = trade_row.get_card(i);
                println!("{} has been scrapped from the Trade Row", card.name);
                trade_row.scrapped.push(card);
                player.lose_ability(Ability::ScrapFromTradeRow);
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
                player.lose_ability(Ability::BuyTopDeck);
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
                let ability = Ability::Or(Box::new(a.to_ability()), Box::new(b.to_ability()));
                let choice = match first {
                    true => { *a },
                    false => { *b }
                };
                println!("{} chooses {:?}", player.name, choice);
                player.lose_ability(ability);
                player.gain_ability(choice.to_ability());
            },
            Choice::ScrapDiscard(i) => {
                let card = player.discard.remove(i);
                println!("{} scraps {} from discard", player.name, card.name);
                player.scrapped.push(card);
                player.lose_ability(Ability::ScrapDiscard);
            },
            Choice::ScrapHand(i) => {
                let card = player.hand.remove(i);
                println!("{} scraps {} from hand", player.name, card.name);
                player.scrapped.push(card);
                player.mandatory_abilities.pop().unwrap();
            },
            Choice::ScrapDiscardDraw(i) => {
                let scrapped = player.discard.remove(i);
                println!("{} scraps {} from discard and draws a card", player.name, scrapped.name);
                player.scrapped.push(scrapped);
                player.effects.push(Effect::Draw);
                player.lose_ability(Ability::ScrapDiscardDraw);
            },
            Choice::ScrapHandDraw(i) => {
                let card = player.hand.remove(i);
                println!("{} scraps {} from hand and draws a card", player.name, card.name);
                player.effects.push(Effect::Draw);
                player.scrapped.push(card);
                player.lose_ability(Ability::ScrapHandDraw);
            },
            Choice::AndOr(a, b, first, second) => {
                println!("{} chooses between {:?} and/or {:?}", player.name, a, b);
                let ability = Ability::AndOr(Box::new(a.to_ability()), Box::new(b.to_ability()));
                if first {
                    println!("{} chooses {:?}", player.name, a);
                    player.gain_ability(a.to_ability());
                }
                if second {
                    println!("{} chooses {:?}", player.name, b);
                    player.gain_ability(b.to_ability());
                }
                println!("{:?}", player.abilities);
                player.lose_ability(ability);
            },
            Choice::ScrapSelf(i) => {
                let card = player.in_play.remove(i);
                println!("{} scraps {}", player.name, card.name);
                for ability in &card.scrap_abilities {
                    player.gain_ability(ability.clone());
                }
                player.effects.extend(card.scrap_effects.clone());
                player.scrapped.push(card);
            },
            Choice::BlobDraw(n) => {
                println!("{} draws {} cards from Blob World", player.name, n);
                for _ in 0..n {
                    player.effects.push(Effect::Draw);
                }
                player.lose_ability(Ability::BlobDraw);
            },
            // Scrap then draw. Used by Brain World.
            Choice::ScrapDraw(n) => {
                for _ in 0..n {
                    player.gain_ability(
                        Ability::Or(
                            Box::new(Ability::ScrapDiscardDraw),
                            Box::new(Ability::ScrapHandDraw),
                        )
                    );
                }
            },
            // Draw then scrap. Used by Machine Base.
            Choice::DrawThenScrap => {
                println!("{} chooses to draw and then scrap from hand", player.name);
                player.effects.push(Effect::Draw);
                player.mandatory_abilities.push(Ability::ScrapHand);
                player.lose_ability(Ability::DrawThenScrap);
            },
            Choice::GainAuthority(n) => {
                player.effects.push(Effect::GainAuthority(n));
                player.lose_ability(Ability::GainAuthority(n));
            },
            Choice::GainCombat(n) => {
                player.effects.push(Effect::GainCombat(n));
                player.lose_ability(Ability::GainCombat(n));
            },
            Choice::GainTrade(n) => {
                player.effects.push(Effect::GainTrade(n));
                player.lose_ability(Ability::GainTrade(n));
            },
            Choice::CopyShip(i) => {
                let mut ship_to_copy = player.in_play[i].clone();
                println!("{}'s Stealth Needle copies {}",
                         player.name,
                         ship_to_copy.name);
                ship_to_copy.run(player, opponents, trade_row);
                if ship_to_copy.faction == Faction::Blob {
                    player.blobs_played_this_turn += 1;
                }
                player.lose_ability(Ability::CopyShip);
            },
            _ => (),
        }
        player.process_effects(&trade_row, opponents);
    }

    pub fn to_ability(&self) -> Ability {
        match self {
            Choice::Attack(_) => Ability::Attack,
            Choice::AcquireFromTradeRow(_) => Ability::AcquireFromTradeRow,
            Choice::AndOr(a, b, _, _) => Ability::AndOr(
                Box::new(a.to_ability()),
                Box::new(b.to_ability()),
            ),
            Choice::BlobDraw(_) => Ability::BlobDraw,
            Choice::Buy(_) => Ability::Buy,
            Choice::BuyTopDeck(_) => Ability::BuyTopDeck,
            Choice::CopyShip(_) => Ability::CopyShip,
            Choice::DestroyBase(_, _) => Ability::DestroyBase,
            Choice::DiscardCard(_) => Ability::DiscardCard,
            Choice::DiscardCardDraw(_) => Ability::DiscardCardDraw,
            Choice::DiscardThenDraw(_) => Ability::DiscardThenDraw,
            Choice::DrawThenScrap => Ability::DrawThenScrap,
            Choice::EndTurn => Ability::EndTurn,
            Choice::GainAuthority(n) => Ability::GainAuthority(*n),
            Choice::GainCombat(n) => Ability::GainCombat(*n),
            Choice::GainTrade(n) => Ability::GainTrade(*n),
            Choice::Or(a, b, _) => Ability::Or(
                Box::new(a.to_ability()),
                Box::new(b.to_ability())
            ),
            Choice::Play(_) => Ability::Play,
            Choice::ScrapDiscard(_) => Ability::ScrapDiscard,
            Choice::ScrapDiscardDraw(_) => Ability::ScrapDiscardDraw,
            Choice::ScrapDraw(_) => Ability::ScrapDraw,
            Choice::ScrapFromTradeRow(_) => Ability::ScrapFromTradeRow,
            Choice::ScrapHand(_) => Ability::ScrapHand,
            Choice::ScrapHandDraw(_) => Ability::ScrapHandDraw,
            Choice::ScrapSelf(_) => Ability::ScrapSelf,
        }
    }
}

impl Ability {
    pub fn to_choice(&self) -> Choice {
        match self {
            Ability::Attack => Choice::Attack(0),
            Ability::AcquireFromTradeRow => Choice::AcquireFromTradeRow(0),
            Ability::AndOr(a, b) => Choice::AndOr(
                Box::new(a.to_choice()),
                Box::new(b.to_choice()),
                false,
                false
            ),
            Ability::BlobDraw => Choice::BlobDraw(0),
            Ability::Buy => Choice::Buy(0),
            Ability::BuyTopDeck => Choice::BuyTopDeck(0),
            Ability::CopyShip => Choice::CopyShip(0),
            Ability::DestroyBase => Choice::DestroyBase(0, 0),
            Ability::DiscardCard => Choice::DiscardCard(0),
            Ability::DiscardCardDraw => Choice::DiscardCardDraw(0),
            Ability::DiscardThenDraw => Choice::DiscardThenDraw(0),
            Ability::DrawThenScrap => Choice::DrawThenScrap,
            Ability::EndTurn => Choice::EndTurn,
            Ability::GainAuthority(n) => Choice::GainAuthority(*n),
            Ability::GainCombat(n) => Choice::GainCombat(*n),
            Ability::GainTrade(n) => Choice::GainTrade(*n),
            Ability::Or(a, b) => Choice::Or(
                Box::new(a.to_choice()),
                Box::new(b.to_choice()),
                true
            ),
            Ability::Play => Choice::Play(0),
            Ability::ScrapDiscard => Choice::ScrapDiscard(0),
            Ability::ScrapDiscardDraw => Choice::ScrapDiscardDraw(0),
            Ability::ScrapDraw => Choice::ScrapDraw(0),
            Ability::ScrapFromTradeRow => Choice::ScrapFromTradeRow(0),
            Ability::ScrapHand => Choice::ScrapHand(0),
            Ability::ScrapHandDraw => Choice::ScrapHandDraw(0),
            Ability::ScrapSelf => Choice::ScrapSelf(0),
        }
    }

    pub fn expand(&self,
                  player: &Player,
                  opponents: &[&mut Player],
                  trade_row: &TradeRow) -> Vec<Choice> {
        let mut choices: Vec<Choice> = Vec::new();

        match self {
            Ability::Or(a, b) => {
                for choice in a.expand(player, opponents, trade_row) {
                    choices.push(Choice::Or(Box::new(choice), Box::new(b.to_choice()), true))
                }
                for choice in b.expand(player, opponents, trade_row) {
                    choices.push(Choice::Or(Box::new(a.to_choice()), Box::new(choice), false))
                }
            }
            Ability::AndOr(a, b) => {
                for choice in a.expand(player, opponents, trade_row) {
                    choices.push(
                        Choice::AndOr(Box::new(choice.clone()), Box::new(b.to_choice()), true, false)
                    );

                    for other_choice in b.expand(player, opponents, trade_row) {
                        choices.push(Choice::AndOr(Box::new(choice.clone()), Box::new(other_choice), true, true));
                    }
                }
                for choice in b.expand(player, opponents, trade_row) {
                    choices.push(Choice::AndOr(
                            Box::new(a.to_choice()),
                            Box::new(choice),
                            false,
                            true
                            ))
                }
            }
            Ability::Play => {
                for (i, _card) in player.hand.iter().enumerate() {
                    choices.push(Choice::Play(i));
                }
            },
            Ability::Attack => {
                if player.combat > 0 {
                    for (i, _opponent) in opponents.iter().enumerate() {
                        choices.push(Choice::Attack(i));
                    }
                }
            }
            Ability::Buy => {
                for (i, card) in trade_row.face_up.iter().enumerate() {
                    if player.trade >= card.cost {
                        choices.push(Choice::Buy(i));
                    }
                }
            }
            Ability::CopyShip => {
                for (i, card) in player.in_play.iter().enumerate() {
                    if card.card_type == CardType::Ship &&
                       card.ship_type != ShipType::StealthNeedle {
                        choices.push(Choice::CopyShip(i));
                    }
                }
            }
            Ability::DiscardCard => {
                for (i, _card) in player.hand.iter().enumerate() {
                    choices.push(Choice::DiscardCard(i));
                }
            }
            Ability::GainAuthority(n) => {
                choices.push(Choice::GainAuthority(*n));
            }
            Ability::GainTrade(n) => {
                choices.push(Choice::GainTrade(*n));
            }
            Ability::GainCombat(n) => {
                choices.push(Choice::GainCombat(*n));
            }
            Ability::ScrapFromTradeRow => {
                for (i, _card) in trade_row.face_up.iter().enumerate() {
                    choices.push(Choice::ScrapFromTradeRow(i));
                }
            }
            Ability::AcquireFromTradeRow => {
                for (i, _card) in trade_row.face_up.iter().enumerate() {
                    choices.push(Choice::AcquireFromTradeRow(i));
                }
            }
            Ability::BlobDraw => {
                choices.push(Choice::BlobDraw(player.blobs_played_this_turn))
            }
            Ability::BuyTopDeck => {
                for (i, card) in trade_row.face_up.iter().enumerate() {
                    if player.trade >= card.cost {
                        choices.push(Choice::BuyTopDeck(i));
                    }
                }
            }
            Ability::ScrapHand => {
                for (i, _card) in player.hand.iter().enumerate() {
                    choices.push(Choice::ScrapHand(i));
                }
            }
            Ability::ScrapDiscard => {
                for (i, _card) in player.discard.iter().enumerate() {
                    choices.push(Choice::ScrapDiscard(i));
                }
            }
            Ability::DrawThenScrap => {
                choices.push(Choice::DrawThenScrap);
            }
            Ability::ScrapDraw => {
                for (i, _card) in player.hand.iter().enumerate() {
                    choices.push(Choice::ScrapDraw(i));
                }
            }
            Ability::EndTurn => {
                choices.push(Choice::EndTurn);
            }
            Ability::ScrapSelf => {
                for (i, card) in player.in_play.iter().enumerate() {
                    if !card.scrap_abilities.is_empty() || !card.scrap_effects.is_empty() {
                        choices.push(Choice::ScrapSelf(i));
                    }
                }
            },
            Ability::DestroyBase => {
                for (i, opponent) in opponents.iter().enumerate() {
                    for (c, _base) in opponent.in_play.iter().enumerate() {
                        choices.push(Choice::DestroyBase(i, c));
                    }
                }
            },
            Ability::DiscardThenDraw => {
                for (i, _card) in player.hand.iter().enumerate() {
                    choices.push(Choice::DiscardThenDraw(i));
                }
            }
            Ability::DiscardCardDraw => {
                for (i, _card) in player.hand.iter().enumerate() {
                    choices.push(Choice::DiscardCardDraw(i));
                }
            }
            Ability::ScrapDiscardDraw => {
                for (i, _card) in player.discard.iter().enumerate() {
                    choices.push(Choice::ScrapDiscardDraw(i));
                }
            }
            Ability::ScrapHandDraw => {
                for (i, _card) in player.hand.iter().enumerate() {
                    choices.push(Choice::ScrapHandDraw(i));
                }
            }
            _ => panic!("Don't know how to expand this ability: {:?}", self)
        };

        choices
    }
}

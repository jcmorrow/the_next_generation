use player::Player;
use choice::Choice;

pub struct Event<'a> {
    player: &'a Player,
    choice: &'a Choice,
    log: String,
}

impl<'a> Event<'a> {
    pub fn new(player: &'a Player, choice: &'a Choice, log: &str) -> Event<'a> {
        Event{
            player: player,
            choice: choice,
            log: log.to_string(),
        }
    }
}

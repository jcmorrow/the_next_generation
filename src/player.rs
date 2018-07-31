const STARTING_AUTHORITY: i32 = 50;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub authority: i32
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player{
            name: name.to_string(),
            authority: STARTING_AUTHORITY
        }
    }
}

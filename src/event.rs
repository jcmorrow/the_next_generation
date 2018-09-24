use card::Card;

#[derive(Debug)]
pub struct Event<'a> {
    pub card: Option<&'a Card>
}


#[castle_api::castle_macro(Type)]
pub struct Card {
    pub id: String,
    pub last4: String,
    pub name: String,
    pub brand: String,
}

impl Card {
    fn id(&self) -> &str {
        &self.id
    }
}
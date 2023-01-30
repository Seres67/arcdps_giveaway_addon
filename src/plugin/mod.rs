pub struct Plugin {
    pub squad: Vec<String>,
    pub player: String,
    pub winner: String,
}

impl Plugin {
    pub fn new() -> Self {
        Self {
            squad: Vec::new(),
            player: String::new(),
            winner: String::from("No winner yet!"),
        }
    }
}
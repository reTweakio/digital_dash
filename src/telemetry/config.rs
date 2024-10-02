pub enum Game {
    Forza,
}

impl Game {
    pub fn detect_game() -> Game {
        Game::Forza
    }
}

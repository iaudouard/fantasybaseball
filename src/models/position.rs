pub enum PositionType {
    Pitcher,
    Batter,
}

pub enum Position {
    SS,
}

impl Position {
    pub fn position_type(&self) -> PositionType {
        match self {
            Position::SS => PositionType::Batter,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Position::SS => String::from("Shortstop"),
        }
    }
}

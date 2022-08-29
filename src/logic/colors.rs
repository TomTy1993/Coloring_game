#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Colors {
    Red = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
}

impl ToString for Colors {
    fn to_string(&self) -> String {
        String::from(match self {
                Colors::Red => "Red",
                Colors::Green => "Green",
                Colors::Blue => "Blue",
                Colors::Yellow => "Yellow"
            }
        )
    }
}

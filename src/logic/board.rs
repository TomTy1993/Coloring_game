use super::colors::Colors;
use rand::Rng;

#[derive(Debug)]
pub struct Board{
    pub state: Vec<Vec<Colors>>,
    pub width: u8,
    pub height: u8
}

impl Board {
    pub fn new(width: u8, height: u8) -> Self {
        let mut rng = rand::thread_rng();
        let mut state = vec![vec![Colors::Red; height as usize]; width as usize];
        for row in state.iter_mut() {
            for cell in row.iter_mut() {
                (*cell) = match rng.gen_range(0..4) {
                    0 => Colors::Red,
                    1 => Colors::Green,
                    2 => Colors::Blue,
                    3 => Colors::Yellow,
                    _ => Colors::Yellow
                }
            }
        }
        Board { state, width, height }
    }

    pub fn calculate_next_board_state(&mut self, next_color: &Colors) {
        let previous_color = self.state[0][0];
        let mut previous_location = Vec::<(u8, u8)>::new();
        self.internal_calculate_next_board_state(0, 0, &mut previous_location, &previous_color, next_color);
    }

    fn internal_calculate_next_board_state(&mut self, x: u8, y: u8,
        previous_location: &mut Vec::<(u8, u8)>,
        previous_color: &Colors,
        next_color: &Colors) {
        if x >= self.width || y >= self.height {
            return;
        }
        if previous_location.contains(&(x, y)){
            return;
        }
        previous_location.push((x, y));

        if self.state[x as usize][y as usize] == *previous_color {
            self.state[x as usize][y as usize] = *next_color;
        }
        else {
            return;
        }

        self.internal_calculate_next_board_state(x + 1, y, previous_location, &previous_color, next_color);
        if x > 0{
            self.internal_calculate_next_board_state(x - 1, y, previous_location, &previous_color, next_color);
        }
        self.internal_calculate_next_board_state(x, y + 1, previous_location, &previous_color, next_color);
        if y > 0 {
            self.internal_calculate_next_board_state(x, y - 1, previous_location, &previous_color, next_color);
        }
    }
}
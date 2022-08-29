use super::board::Board;
use super::colors::Colors;

#[derive(Debug)]
pub struct GameManager{
    pub board: Board,
    pub total_tryies: u32,
    pub tryies_left: u32,
    pub last_colos_used: Colors,
    pub game_state: GameState
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GameState{
    InProgress,
    Won,
    Lost
}

impl ToString for GameState {
    fn to_string(&self) -> String {
        String::from(match self {
                GameState::InProgress => "InProgress",
                GameState::Won => "Won",
                GameState::Lost => "Lost"
            }
        )
    }
}

impl GameManager {
    pub fn new(number_of_tries: u32, board_width: u8, board_height: u8) -> Self {
        let board = Board::new(board_width, board_height);
        let last_colos_used = board.state[0][0];
        GameManager { 
            board: board,
            total_tryies: number_of_tries,
            tryies_left: number_of_tries,
            last_colos_used: last_colos_used,
            game_state: GameState::InProgress 
        }
    }

    pub fn reset_game(&mut self){
        self.board = Board::new(self.board.width, self.board.height);
        self.tryies_left = self.total_tryies;
        self.last_colos_used = self.board.state[0][0];
        self.game_state = GameState::InProgress;
    }

    pub fn make_move(&mut self, next_color: &Colors) -> bool {
        if self.tryies_left <= 0 {
            return false;
        }

        if self.last_colos_used == *next_color {
            return false;
        }
        
        self.tryies_left -= 1;
        self.board.calculate_next_board_state(next_color);
        self.last_colos_used = *next_color;

        if Self::check_win_condition(&self.board) {
            self.game_state = GameState::Won;
        }
        else if self.tryies_left <= 0 {
            self.game_state = GameState::Lost;
        }

        return true;
    }

    pub fn check_win_condition(board: &Board) -> bool {
        let first_cell = board.state[0][0];
        for row in board.state.iter() {
            for cell in row.iter() {
                if first_cell != *cell{
                    return false;
                }
            }
        }

        return true;
    }
}
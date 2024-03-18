fn main() {
    println!("Hello, world!");
}

enum CaseState {
    Green,
    Blue,
    Red,
    Bomb,
    Rupoor,
    Unknow,
}
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

struct Grid{
    grid: Vec<Vec<CaseState>>,
    bomb_number: u8,
    rupoor_number: u8,
}

impl Grid{
    fn new(difficulty: Difficulty)->Grid {
        match difficulty {
            Difficulty::Easy => Grid { grid: vec![vec![CaseState::Unknow; 5]; 4], bomb_number: 4, rupoor_number: 0 },
            Difficulty::Normal => Grid { grid: vec![vec![CaseState::Unknow; 6]; 4], bomb_number: 4, rupoor_number: 4 },
            Difficulty::Hard => Grid { grid: vec![vec![CaseState::Unknow; 8]; 5], bomb_number: 8, rupoor_number: 8 },
        }
    }

    fn update(&mut self, x: usize, y: usize, state: CaseState){
        self.grid[x][y] = state;
    }
}
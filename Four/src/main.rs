use std::collections::HashMap;
use std::fs;

struct BoardPosition {
    row: usize,
    col: usize
}

struct Token {
    value: u32,
    marked: bool
}

struct Board {
    positions: Vec<Vec<Token>>,
    value_positions: HashMap<u32, Vec<BoardPosition>>
}

impl Board {
    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for row in &self.positions {
            for column in row {
                if !column.marked {
                    sum = sum + column.value;
                }
            }
        }
        sum
    }
    fn to_string(&self) -> String {
        let mut board_str = String::new();
        for row in &self.positions {
            for (i, token) in row.iter().enumerate() {
                if i != 0 {
                    board_str.push(',')
                }
                if token.marked {
                    board_str.push('X')
                } else {
                    board_str.push_str(token.value.to_string().as_str());
                }
            }
            board_str.push('\n');
        }
        board_str
    }

    fn from_string_list(lines: &Vec<String>) -> Result<Board, ()> {
        let mut board = Board{ positions: vec![], value_positions: Default::default() };
        for (row_index, line) in lines.iter().enumerate() {
            let row_values = line.split_whitespace().collect::<Vec<&str>>();
            let mut row = Vec::new();
            for (col_index, value_str) in row_values.iter().enumerate() {
                match value_str.parse::<u32>() {
                    Ok(value) => {
                        row.push(Token{ value, marked: false });
                        match board.value_positions.get_mut(&value) {
                            None => {
                                board.value_positions.insert(value, vec![BoardPosition{ row: row_index, col: col_index }]);
                            }
                            Some(value_position) => {
                                value_position.push(BoardPosition{ row: row_index, col: col_index });
                            }
                        }

                    }
                    Err(_) => return Err(())
                }
            }
            board.positions.push(row);
        }
        Ok(board)
    }

    fn play_move(&mut self, move_value: u32) -> bool {
        let mut r_val = false;
        match self.value_positions.get(&move_value) {
            None => r_val = false,
            Some(positions) => {
                for pos in positions {
                    let row = match self.positions.get_mut(pos.row) {
                        None => {
                            println!("Error getting the row by index {}", pos.row);
                            return false;
                        },
                        Some(r) => r
                    };
                    match row.get_mut(pos.col) {
                        None => {
                            println!("Error getting the column {}", pos.col);
                            return false;
                        },
                        Some(token) => {
                            token.marked = true;
                        }
                    }
                    let mut row_complete = true;
                    for pos in row {
                        row_complete &= pos.marked
                    }
                    if row_complete {
                        r_val = true;
                    }
                    let mut column_complete = true;
                    for row in &self.positions {
                        match row.get(pos.col) {
                            None => continue,
                            Some(col_val) => {
                                column_complete &= col_val.marked;
                            }
                        }
                    }
                    if column_complete {
                        r_val = true;
                    }
                }
            }
        }
        r_val
    }
}

trait MutableRetain<T> {
    fn retain_mut<F>(&mut self, test : F) where F: Fn(&mut T, usize, usize) -> bool;
}

impl <T> MutableRetain<T> for Vec<T> {
    fn retain_mut<F>(&mut self, test: F) where F: Fn(&mut T, usize, usize) -> bool {
        let mut current_index = 0;
        let mut current_length = self.len();
        while let Some(value) = self.get_mut(current_index) {
            let retain = test(value, current_index, current_length);
            if retain {
                current_index += 1;
            } else {
                self.remove(current_index);
                current_length -= 1;
            }
        }
    }
}

struct Game {
    moves: Vec<u32>,
    boards: Vec<Board>
}

impl Game {
    fn from_chunks(chunks: &Vec<Vec<String>>) -> Result<Game, ()> {
        let mut chunk_iter = chunks.iter();
        let move_chunk = match chunk_iter.next() {
            None => return Err(()),
            Some(mc) => mc
        };
        let move_line = match move_chunk.first() {
            None => return Err(()),
            Some(ml) => ml
        };
        let mut moves = Vec::new();
        for value_str in move_line.split(",") {
            match value_str.parse::<u32>() {
                Ok(value) => {
                    moves.push(value);
                }
                Err(_) => return Err(())
            }
        }
        let mut boards = Vec::new();

        for chunk in chunk_iter {
            match Board::from_string_list(chunk) {
                Ok(b) => boards.push(b),
                Err(_) => return Err(())
            }
        }
        Ok(Game{ moves, boards })
    }

    fn play_one(&mut self) {
        for move_value in &self.moves {
            for board in self.boards.iter_mut() {
                let win = board.play_move(*move_value);
                if win {
                    let unmarked_sum = board.sum_unmarked();
                    println!("Play 1: won on move {} with unmarked sum of {} and final score of {}", move_value, unmarked_sum, unmarked_sum * move_value);
                    return;
                }
            }
        }
    }
    fn play_two(&mut self) {
        for move_value in &self.moves {
            self.boards.retain_mut(|board, _, length| {
                let win = board.play_move(*move_value);
                if win {
                    if length == 1 {
                        let unmarked_sum = board.sum_unmarked();
                        println!("Play 2: won on move {} with unmarked sum of {} and final score of {}", move_value, unmarked_sum, unmarked_sum * move_value);
                    }
                    return false;
                }
                return true;
            });
            if self.boards.is_empty() {
                return;
            }
        }
    }
}

fn read_chunks(file_path: &str) -> Result<Vec<Vec<String>>, ()> {
    let contents = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return Err(())
    };
    if contents.is_empty() {
        return Err(());
    }
    let mut chunks = Vec::new();
    for chunk in contents.split("\n\n") {
        let mut lines = Vec::new();
        for line in chunk.split("\n") {
            lines.push(String::from(line));
        }
        chunks.push(lines);
    }
    Ok(chunks)
}

fn main() {
    let chunks = read_chunks("input").expect("Couldn't load file");
    let mut game = Game::from_chunks(&chunks).expect("Couldn't load the game");
    let mut game2 = Game::from_chunks(&chunks).expect("Couldn't load the game");
    game.play_one();
    game2.play_two();
}

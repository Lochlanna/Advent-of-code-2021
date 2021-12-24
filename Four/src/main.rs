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
    fn from_string_list(lines: &Vec<String>) -> Result<Board, ()> {
        let mut board = Board{ positions: vec![], value_positions: Default::default() };
        for (row_index, line) in lines.iter().enumerate() {
            let row_values = line.split(" ");
            let mut row = Vec::new();
            for (col_index, value_str) in row_values.enumerate() {
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
        match self.value_positions.get(&move_value) {
            None => return false,
            Some(positions) => {
                for pos in positions {
                    let row = match self.positions.get_mut(pos.row) {
                        None => return false,
                        Some(r) => r
                    };
                    match row.get_mut(pos.col) {
                        None => return false,
                        Some(token) => {
                            token.marked = true;
                        }
                    }
                    let mut row_complete = true;
                    for pos in row {
                        if !pos.marked {
                            row_complete = false;
                        }
                    }
                    if row_complete {
                        return true;
                    }
                }
            }
        }
        for row in self.positions {

        }
        false
    }
}

struct Game {
    moves: Vec<u32>,
    boards: Vec<Board>
}

impl Game {
    fn from_chunks(chunks: &Vec<Vec<String>>) -> Result<Game, ()> {
        let move_chunk = match chunks.first() {
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
        let mut chunk_iter = chunks.iter();
        chunk_iter.next(); //skip the move chunk
        for chunk in chunk_iter {
            match Board::from_string_list(chunk) {
                Ok(b) => boards.push(b),
                Err(_) => return Err(())
            }
        }
        Ok(Game{ moves, boards })
    }

    fn play(&mut self) {
        for move_value in self.moves {
            for board in &mut self.boards {
                let win = board.play_move(value);
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
    let game = Game::from_chunks(&chunks).expect("Couldn't load the game");
    println!("Hello, world!");
}

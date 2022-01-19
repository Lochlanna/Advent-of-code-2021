use std::fmt::{Display, Formatter};
use std::time::Instant;
use crate::table::{Table, TableCell, TableIterator, TableIteratorType, IteratorDirection};

mod table;

impl<T: Copy> Table<T> {
    fn fold(&mut self, mut src_iter: TableIterator, mut dst_iter: TableIterator) {
        loop {
            let src_value:T = match src_iter.next(self) {
                None => break, // There is no more to iterate over
                Some(src_value) => {
                    match src_value {
                        TableCell::Null => { // empty cell so we skip it on the destination side
                            if !dst_iter.skip(self) {
                                break; // no more space on the destination
                            }
                            continue; // We have skipped here so we need to go back to the start of the loop
                        },
                        TableCell::Some(src_value) => *src_value
                    }
                }
            };
            if !dst_iter.update(self, src_value) {
                break;
            }
        }
    }
    fn fold_x(&mut self, x: usize) {
        if x + 1 >= self.num_columns() {
            // Can't fold along a row that doesn't exist or the last row
            return;
        }
        let src_iter = table::TableIterator::new(x+1, IteratorDirection::Forward, TableIteratorType::Column);
        let dst_iter = table::TableIterator::new(x-1, IteratorDirection::Backward, TableIteratorType::Column);

        self.fold(src_iter, dst_iter);
        self.truncate_by_column(x);
    }

    fn fold_y(&mut self, y: usize) {
        if y + 1 >= self.num_rows() {
            // Can't fold along a row that doesn't exist or the last row
            return;
        }
        let src_iter = table::TableIterator::new(y+1, IteratorDirection::Forward, TableIteratorType::Row);
        let dst_iter = table::TableIterator::new(y-1, IteratorDirection::Backward, TableIteratorType::Row);

        self.fold(src_iter, dst_iter);
        self.truncate_by_row(y);
    }

    fn count_non_null_cells(&self) -> usize {
        let mut non_null:usize = 0;
        for cell in self.cell_iterator() {
            match cell {
                TableCell::Null => {}
                TableCell::Some(_) => non_null += 1
            }
        }
        non_null
    }
}

impl Display for table::Table<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table_string = String::with_capacity(self.len() + self.num_rows());
        for row in self.row_iterator() {
            for column in row {
                match column {
                    TableCell::Null => table_string.push(' '),
                    TableCell::Some(_) => {table_string.push('█')}
                }
            }
            table_string.push('\n');
        }
        table_string.pop();//pop final new line
        f.write_str(table_string.as_str())
    }
}

enum Fold {
    X(usize),
    Y(usize)
}

fn read_input(filename: &str) -> (Table<bool>, Vec<Fold>) {
    let file_contents = match std::fs::read_to_string(filename) {
        Ok(fc) => fc,
        Err(_) => panic!("Couldn't read the input")
    };
    let mut points:Vec<(usize, usize)> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut line_iterator = file_contents.lines();
    for line in line_iterator.by_ref() {
        if line.is_empty() {
            break; // next lines are the fold instructions
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            panic!("Invalid point while reading input");
        }
        let x = parts[0].parse::<usize>().expect("Couldn't parse point while reading input");
        let y = parts[1].parse::<usize>().expect("Couldn't parse point while reading input");
        points.push((x,y));
        max_x = std::cmp::max(x+1, max_x);
        max_y = std::cmp::max(y+1, max_y);
    }
    let mut table = Table::with_capacity(max_x, max_x * max_y);
    for (x, y) in points {
        table.set_cell(x,y,true);
    }
    let mut instructions = Vec::new();
    for line in line_iterator {
        let line_parts: Vec<&str> = line.split('=').collect();
        if line_parts.len() != 2 || line_parts[0].is_empty() || line_parts[1].is_empty() {
            panic!("couldn't parse instruction");
        }
        let fold_index = line_parts[1].parse::<usize>().expect("Couldn't parse instruction index");
        if line_parts[0].ends_with('x') {
            instructions.push(Fold::X(fold_index));
        } else if line_parts[0].ends_with('y') {
            instructions.push(Fold::Y(fold_index));
        } else {
            panic!("Couldn't parse instruction dimension");
        }
    }
    (table, instructions)
}

fn main() {
    let now = Instant::now();
    let (mut table, instructions) = read_input("input");
    let mut dots_after_first = 0;
    let mut first = true;
    for instruction in instructions {
        match instruction {
            Fold::X(i) => table.fold_x(i),
            Fold::Y(i) => table.fold_y(i),
        }
        if first {
            dots_after_first = table.count_non_null_cells();
            first = false;
        }

    }
    println!("After the first fold there are {} dots. After all the folds there are {} dots which looks like\n{}", dots_after_first, table.count_non_null_cells(), table);
    println!("time taken was {} seconds", now.elapsed().as_secs_f64());
}

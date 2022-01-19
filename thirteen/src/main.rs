use std::fmt::{Display, Formatter};
use crate::table::{Table, TableCell, TableIterator, TableIteratorType, IteratorDirection};

mod table;

trait Foldable {
    /// Folds the structure around the provided x axis
    /// The structure will be folded left to right around the axis
    /// The line along which the struct is folded will no longer exist after this operation
    /// The original table is modified to perform this operation
    ///
    /// # Arguments
    ///
    /// * `x`: The index of the column to fold around. This column will no longer exist after the function is complete
    ///
    /// returns: ()
    fn fold_x(&mut self, x:usize);
    fn fold_y(&mut self, y:usize);
}

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
                        TableCell::Some(src_value) => src_value.clone()
                    }
                }
            };
            if !dst_iter.update(self, src_value) {
                break;
            }
        }
    }
}

impl<T: Copy> Foldable for table::Table<T> {

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
}

impl Display for table::Table<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table_string = String::with_capacity(self.len() + self.num_rows());
        for row in self.row_iterator() {
            for column in row {
                match column {
                    TableCell::Null => table_string.push('.'),
                    TableCell::Some(_) => {table_string.push('#')}
                }
            }
            table_string.push('\n');
        }
        table_string.pop();//pop final new line
        f.write_str(table_string.as_str())
    }
}

fn read_input(filename: &str) -> Table<bool> {
    let file_contents = match std::fs::read_to_string(filename) {
        Ok(fc) => fc,
        Err(_) => panic!("Couldn't read the input")
    };
    let mut points:Vec<(usize, usize)> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in file_contents.lines() {
        if line.is_empty() {
            break;
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
    table
}

fn main() {
    let mut table = read_input("input_small");
    println!("Read input table \n{}", table);
    table.fold_y(7);
    println!("Fold on 7 \n{}", table);
}

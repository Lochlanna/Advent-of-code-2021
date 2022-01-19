use std::slice::{Chunks, Iter};

pub enum TableCell<T> {
    Null,
    Some(T)
}

impl<T: Copy + Clone> Clone for TableCell<T> {
    fn clone(&self) -> Self {
        match self {
            TableCell::Null => TableCell::Null,
            TableCell::Some(v) => TableCell::Some(*v)
        }
    }
}

impl<T:Copy> Copy for TableCell<T> {
}

impl<T> Default for TableCell<T> {
    fn default() -> Self {
        TableCell::Null
    }
}

pub struct Table<T> {
    data: Vec<TableCell<T>>,
    row_size: usize
}

impl <T: Copy> Table<T> {
    // This is kinda pricey
    pub fn truncate_by_column(&mut self, col: usize) {
        let mut new_data_table = Vec::with_capacity(self.num_rows() * col);
        for value in self.data.chunks(self.row_size) {
            new_data_table.extend_from_slice(&value[0..col]);
        }
        self.row_size = col;
        self.data = new_data_table;
    }
}

impl<T> Table<T> {
    pub fn new(row_size: usize) -> Table<T> {
        Table {
            data: vec![],
            row_size
        }
    }

    pub fn with_capacity(row_size: usize, capacity: usize) -> Table<T> {
        let mut table = Table::new(row_size);
        table.data.resize_with(capacity, Default::default);
        table
    }

    pub fn row_iterator(&self) -> Chunks<'_, TableCell<T>> {
        self.data.chunks(self.row_size)
    }

    pub fn cell_iterator(&self) -> Iter<'_, TableCell<T>> {
        self.data.iter()
    }

    //mmm cheap
    pub fn truncate_by_row(&mut self, row: usize) {
        let start_of_row = row * self.row_size;
        self.data.truncate(start_of_row);
    }

    pub fn set_cell(&mut self, x: usize, y:usize, value: T) {
        let insert_index = y * self.row_size + x;
        self.set_cell_by_index(insert_index, value);
    }

    pub fn set_cell_by_index(&mut self, index: usize, value: T) {
        if self.data.len() < index+1 {
            self.data.resize_with(index+1, Default::default)
        }
        self.data[index] = TableCell::Some(value);
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&TableCell<T>> {
        let index = y * self.row_size + x;
        self.get_index(index)
    }

    pub fn get_mut(&mut self, x:usize, y:usize) -> Option<&mut TableCell<T>> {
        let index = y * self.row_size + x;
        self.get_index_mut(index)
    }

    pub fn num_rows(&self) -> usize {
        (self.data.len() as f64 / self.row_size as f64).ceil() as usize
    }

    pub fn num_columns(&self) -> usize {
        self.row_size
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<&mut TableCell<T>> {
        match self.data.get_mut(index) {
            None => None,
            Some(value) => Some(value),
        }
    }

    fn get_index(&self, index: usize) -> Option<&TableCell<T>> {
        match self.data.get(index) {
            None => None,
            Some(value) => Some(value),
        }
    }
}
pub enum IteratorDirection{
    Forward,
    Backward
}

pub enum TableIteratorType{
    Row,
    Column
}

pub struct TableIterator {
    current_row:usize,
    current_col:usize,
    direction: IteratorDirection,
    iterator_type: TableIteratorType
}

pub enum UpdateStatus {
    Updated,
    NotUpdated,
    Failed
}

impl TableIterator {
    pub fn new(start_index:usize, direction: IteratorDirection, iterator_type: TableIteratorType) -> TableIterator {
        match iterator_type {
            TableIteratorType::Row => {
                TableIterator {
                    current_row: start_index,
                    current_col: 0,
                    direction,
                    iterator_type
                }
            }
            TableIteratorType::Column => {
                TableIterator {
                    current_row: 0,
                    current_col: start_index,
                    direction,
                    iterator_type
                }
            }
        }

    }

    fn increment_column<T>(&mut self, table: &Table<T>) -> Option<usize> {
        if self.current_row >= table.num_rows() {
            self.current_row = 0;
            match self.direction {
                IteratorDirection::Forward => self.current_col += 1,
                IteratorDirection::Backward => {
                    if self.current_col > 0 {
                        self.current_col -= 1;
                    } else {
                        return None
                    }
                }
            };
        }
        let current_index = self.current_row * table.row_size + self.current_col;
        if current_index > table.len() {
            return None;
        }
        self.current_row += 1;
        Some(current_index)
    }

    fn increment_row<T>(&mut self,table: &Table<T>) -> Option<usize> {
        if self.current_col >= table.row_size {
            self.current_col = 0;
            match self.direction {
                IteratorDirection::Forward => self.current_row += 1,
                IteratorDirection::Backward => {
                    if self.current_row > 0 {
                        self.current_row -= 1;
                    } else {
                        return None;
                    }
                }
            }
        }
        let current_index = self.current_row * table.row_size + self.current_col;
        if current_index > table.len() {
            return None;
        }
        self.current_col += 1;
        Some(current_index)
    }

    fn increment<T>(&mut self,table: &Table<T>) -> Option<usize> {
        match self.iterator_type {
            TableIteratorType::Row => self.increment_row(table),
            TableIteratorType::Column => self.increment_column(table)
        }
    }

    pub fn next<'a, T>(&mut self, table: &'a Table<T>) -> Option<&'a TableCell<T>> {
        match self.increment(table) {
            None => None,
            Some(index) => table.get_index(index)
        }
    }

    pub fn update<T>(&mut self, table: &mut Table<T>, new_value: T) -> bool {
        match self.increment(table) {
            None => false,
            Some(index) => {
                table.set_cell_by_index(index, new_value);
                true
            }
        }
    }

    pub fn skip<T>(&mut self,  table: &Table<T>) -> bool {
        self.increment(table).is_some()
    }

    pub fn update_if<T>(&mut self,  table: &mut Table<T>, new_value: T, test:fn(&TableCell<T>)->bool) -> UpdateStatus {
        match self.increment(table) {
            None => UpdateStatus::Failed,
            Some(index) => {
                match table.get_index(index) {
                    None => return UpdateStatus::Failed,
                    Some(cell) => {
                        if !test(cell) {
                            return UpdateStatus::NotUpdated;
                        }
                    }
                }
                table.set_cell_by_index(index, new_value);
                UpdateStatus::Updated
            }
        }
    }
}

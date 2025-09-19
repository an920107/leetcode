pub struct Spreadsheet {
    cells: Vec<i32>,
}

impl Spreadsheet {
    pub fn new(rows: i32) -> Self {
        Self {
            cells: vec![0; rows as usize * 26],
        }
    }

    pub fn set_cell(&mut self, cell: String, value: i32) {
        let index = Spreadsheet::count_index(&cell);
        self.cells[index] = value;
    }

    pub fn reset_cell(&mut self, cell: String) {
        self.set_cell(cell, 0);
    }

    pub fn get_value(&self, formula: String) -> i32 {
        let mut result = 0;

        for cell in formula[1..].split('+') {
            if let Ok(val) = cell.parse::<i32>() {
                result += val
            } else {
                let index = Spreadsheet::count_index(cell);
                result += self.cells[index];
            }
        }

        result
    }

    fn count_index(cell: &str) -> usize {
        let col = (cell[..1].as_bytes()[0] - b'A') as usize;
        let row = cell[1..].parse::<usize>().unwrap() - 1;
        row * 26 + col
    }
}

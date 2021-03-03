use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead,
    Alive,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Alive => write!(f, "ALIVE"),
            Cell::Dead  => write!(f, "DEAD"),
        }
    }
}

pub struct Grid {
    num_rows: u32,
    num_cols: u32,
    grid: Vec<Cell>,
}

impl Grid {
    pub fn new(num_rows: u32, num_cols: u32) -> Grid {
        Grid {
            num_rows: num_rows,
            num_cols: num_cols,
            grid: vec![Cell::Dead; (num_rows * num_cols) as usize],
        }
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.grid[idx] = Cell::Alive;
        }
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        ((row * self.num_cols) + col) as usize
    }

    fn get(&self, row: u32, col: u32) -> Cell {
        let index = self.get_index(row, col);
        self.grid[index]
    }

    fn set(&mut self, row: u32, col: u32, state: Cell) {
        let index = self.get_index(row, col);
        self.grid[index] = state
    }

    fn is_cell_alive(&self, row: u32, col: u32) -> bool {
        self.get(row, col) == Cell::Alive
    }

    fn is_any_cell_alive(&self) -> bool {
        let mut alive = false;

        for &cell in self.grid.iter() {
            if cell == Cell::Alive {
                alive = true;
                break;
            }
        }
        
        return alive;
    }

    fn get_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count: u8 = 0;

        let mut new_row: u32;
        let mut new_col: u32;

        // 0 1 2
        // 3 X 4
        // 5 6 7

        // check the top left neighbor
        if (row > 0) && (col > 0) {
            new_row = row - 1;
            new_col = col - 1;

            if self.is_cell_alive(new_row, new_col) { count += 1; }
        }

        // check the top center neighbor
        if row > 0 {
            new_row = row - 1;
            new_col = col;

            if self.is_cell_alive(new_row, new_col) { count += 1; }
        }

        // check the top right neighbor
        if (row > 0) && ((col + 1) < self.num_cols) {
            new_row = row - 1;
            new_col = col + 1;

            if self.is_cell_alive(new_row, new_col) { count += 1; }
        }

        // check left neighbor
        if col > 0 {
            new_row = row;
            new_col = col - 1;

            if self.is_cell_alive(new_row, new_col) { count += 1; }
        }

        // check right neighbor
        if (col + 1) < self.num_cols {
            new_row = row;
            new_col = col + 1;

            if self.is_cell_alive(new_row, new_col) { count += 1; }
        }

        // check bottom left neighbor
        if ((row + 1) < self.num_rows) && (col > 0) {
            new_row = row + 1;
            new_col = col - 1;

            if self.is_cell_alive(new_row, new_col) { count += 1; }
        }

        // check bottom center neighbor
        if (row + 1) < self.num_rows {
            new_row = row + 1;
            new_col = col;

            if self.is_cell_alive(new_row, new_col) { count += 1; }
        }

        // check bottom left neighbor
        if ((row + 1) < self.num_rows) && ((col + 1) < self.num_cols) {
            new_row = row + 1;
            new_col = col + 1;

            if self.is_cell_alive(new_row, new_col) { count += 1; }
        }

        return count;
    }

    fn print(&self) {
        let divider = "-".repeat(((self.num_cols * 2) + 2) as usize);

        println!("{}", divider);
        for row in 0..self.num_rows {
            print!("!");
            for col in 0..self.num_cols {
                match self.get(row, col) {
                    Cell::Alive => print!("* "),
                    Cell::Dead  => print!("  "),
                }
            }
            println!("|");
        }
        println!("{}", divider)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.grid.as_slice().chunks(self.num_cols as usize) {
            for &cell in line {
                let smybol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", smybol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

struct Operation {
    row: u32,
    col: u32,
    state: Cell
}

impl Operation {
    fn new(row: u32, col: u32, state: Cell) -> Operation {
        Operation { row, col, state }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operation[row: {}, col: {}, state: {}]", self.row, self.col, self.state)
    }
}

struct ConwaySim {
    grid: Grid,
    generation: u32,
}

impl ConwaySim {
    fn new(grid: Grid) -> ConwaySim {
        ConwaySim { grid, generation: 0 }
    }

    fn apply_rules(&self, row: u32, col: u32) -> Vec<Operation> {
        let mut operations: Vec<Operation> = Vec::new();

        // determine the number of live neighbors to the current cell
        let neighbor_count = self.grid.get_neighbor_count(row, col);

        // determine if the current cell is alive
        let alive = self.grid.is_cell_alive(row, col);

        // RULES FOR LIVE CELLS ///////////////////////////////////////////////
        if alive {
            // rule 1: any live cell with fewer than two live neighbors dies,
            //          as if caused by under-population.
            if neighbor_count < 2 {
                operations.push(Operation::new(row, col, Cell::Dead));
            }

            // rule 2: any live cell with two or three live neigbors lives on
            //          to the next generation.
            else if neighbor_count <= 3 {
                // do nothing, cell lives
            }

            // rule 3: any live cell with more than three neigborns dies, as if
            //          caused by overcrowding.
            else {
                operations.push(Operation::new(row, col, Cell::Dead));
            }
        }

        // RULES FOR DEAD CELLS ///////////////////////////////////////////////
        else {
            // rule 4: any dead cell with exactly three live neighbors becomes
            //          a live cell, as if by reproduction.
            if neighbor_count == 3 {
                operations.push(Operation::new(row, col, Cell::Alive));
            }
        }

        return operations;
    }

    fn step(&mut self) {
        let mut operations: Vec<Operation> = Vec::new();

        // increment the sim's generation
        self.generation += 1;

        // loop over each cell in the grid
        for row in 0..self.grid.num_rows {
            for col in 0..self.grid.num_cols {
                // apply rules to the cell
                let results = self.apply_rules(row, col);

                // add any resultant operations to the step's operations list
                operations.extend(results);
            }
        }

        // apply any operations for this step to the grid
        for operation in operations {
            self.grid.set(operation.row, operation.col, operation.state)
        }
    }
}

fn main() {
    let mut grid = Grid::new(5, 5);

    grid.set(2, 1, Cell::Alive);
    grid.set(2, 2, Cell::Alive);
    grid.set(2, 3, Cell::Alive);

    let mut sim = ConwaySim::new(grid);

    for _i in 0..105 {
        sim.step();
        println!("Generation: {}", sim.generation);
        // sim.grid.print();
        print!("{}", sim.grid);
        println!("Any cell alive? {}", sim.grid.is_any_cell_alive());
        println!();

        if !sim.grid.is_any_cell_alive() {
            break;
        }

    }
}

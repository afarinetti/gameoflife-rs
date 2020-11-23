use std::fmt;

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    Dead,
    Alive,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CellState::Alive => write!(f, "ALIVE"),
            CellState::Dead  => write!(f, "DEAD"),
        }
    }
}

struct Grid {
    num_rows: usize,
    num_cols: usize,
    grid: Vec<CellState>,
}

impl Grid {
    fn new(num_rows: usize, num_cols: usize) -> Grid {
        let grid = vec![CellState::Dead; num_rows * num_cols];

        Grid { num_rows, num_cols, grid }
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        (row * self.num_cols) + col
    }

    fn get(&self, row: usize, col: usize) -> CellState {
        let index = self.cell_to_index(row, col);
        self.grid[index]
    }

    fn set(&mut self, row: usize, col: usize, state: CellState) {
        let index = self.cell_to_index(row, col);
        self.grid[index] = state
    }

    fn is_cell_alive(&self, row: usize, col: usize) -> bool {
        self.get(row, col) == CellState::Alive
    }

    fn is_any_cell_alive(&self) -> bool {
        let mut alive = false;

        for &cell in self.grid.iter() {
            if cell == CellState::Alive {
                alive = true;
                break;
            }
        }
        
        return alive;
    }

    fn get_neighbor_count(&self, row: usize, col: usize) -> u8 {
        let mut count: u8 = 0;

        let mut new_row: usize;
        let mut new_col: usize;

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
        let divider = "-".repeat((self.num_cols * 2) + 2);

        println!("{}", divider);
        for row in 0..self.num_rows {
            print!("!");
            for col in 0..self.num_cols {
                match self.get(row, col) {
                    CellState::Alive => print!("* "),
                    CellState::Dead  => print!("  "),
                }
            }
            println!("|");
        }
        println!("{}", divider)
    }
}

struct Operation {
    row: usize,
    col: usize,
    state: CellState
}

impl Operation {
    fn new(row: usize, col: usize, state: CellState) -> Operation {
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

    fn apply_rules(&self, row: usize, col: usize) -> Vec<Operation> {
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
                operations.push(Operation::new(row, col, CellState::Dead));
            }

            // rule 2: any live cell with two or three live neigbors lives on
            //          to the next generation.
            else if neighbor_count <= 3 {
                // do nothing, cell lives
            }

            // rule 3: any live cell with more than three neigborns dies, as if
            //          caused by overcrowding.
            else {
                operations.push(Operation::new(row, col, CellState::Dead));
            }
        }

        // RULES FOR DEAD CELLS ///////////////////////////////////////////////
        else {
            // rule 4: any dead cell with exactly three live neighbors becomes
            //          a live cell, as if by reproduction.
            if neighbor_count == 3 {
                operations.push(Operation::new(row, col, CellState::Alive));
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

    grid.set(2, 1, CellState::Alive);
    grid.set(2, 2, CellState::Alive);
    grid.set(2, 3, CellState::Alive);

    let mut sim = ConwaySim::new(grid);

    for _i in 0..105 {
        sim.step();
        println!("Generation: {}", sim.generation);
        sim.grid.print();
        println!("Any cell alive? {}", sim.grid.is_any_cell_alive());
        println!();

        if !sim.grid.is_any_cell_alive() {
            break;
        }

    }
}

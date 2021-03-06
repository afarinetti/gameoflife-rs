use std::fmt;

/// Representation of a Conway's Game of Life [Cell].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    /// [Cell] is dead.
    Dead,

    /// [Cell] is alive.
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

/// Conway's Game of Life game [Grid].
pub struct Grid {
    /// Number of rows (height) of the [Grid].
    num_rows: u32,

    /// Number of columns (width) of the [Grid].
    num_cols: u32,

    /// The game [Grid] represented as a 2D [Vec] in row-major order.
    grid: Vec<Cell>,
}

impl Grid {
    /// Create a new [Grid] instance.
    ///
    /// # Arguments
    /// * `num_rows` - The number of rows (height) of the [Grid].
    /// * `num_cols` - The number of columns (width) of the [Grid].
    pub fn new(num_rows: u32, num_cols: u32) -> Grid {
        Grid {
            num_rows: num_rows,
            num_cols: num_cols,
            grid: vec![Cell::Dead; (num_rows * num_cols) as usize],
        }
    }

    /// Set the cells of the [Grid].
    ///
    /// # Arguments
    /// * `cells` - Array slice of tuples (row, col) to set as [Cell::Alive].
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.cell_to_index(row, col);
            self.grid[idx] = Cell::Alive;
        }
    }

    /// Convert a (row, col) position to a row-major index.
    fn cell_to_index(&self, row: u32, col: u32) -> usize {
        ((row * self.num_cols) + col) as usize
    }

    /// Get a specified [Cell] of the [Grid].
    pub fn get(&self, row: u32, col: u32) -> Cell {
        let index = self.cell_to_index(row, col);
        self.grid[index]
    }

    /// Set a specified [Cell] of the [Grid].
    fn set(&mut self, row: u32, col: u32, state: Cell) {
        let index = self.cell_to_index(row, col);
        self.grid[index] = state
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

/// Represents an [Operation] to be applied to a cell of a [Grid]. 
struct Operation {
    /// Row of the operation.
    row: u32,

    /// Column of the operation.
    col: u32,

    /// The [Cell] state to apply.
    state: Cell
}

impl Operation {
    /// Create a new [Operation].
    fn new(row: u32, col: u32, state: Cell) -> Operation {
        Operation { row, col, state }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operation[row: {}, col: {}, state: {}]", self.row, self.col, self.state)
    }
}

/// Conway's Game of Life Simulation.
pub struct ConwaySim {
    /// Simulation [Grid].
    grid: Grid,

    /// The simulation's current generation.
    generation: u32,
}

impl ConwaySim {
    /// Create a new simulation.
    ///
    /// # Arguments
    /// * `num_rows` - The number of rows (height) of the [Grid].
    /// * `num_cols` - The number of columns (width) of the [Grid].
    pub fn new(num_rows: u32, num_cols: u32) -> ConwaySim {
        ConwaySim {
            grid: Grid::new(num_rows, num_cols),
            generation: 0,
        }
    }

    #[allow(dead_code)]
    pub fn new_with_grid(grid: Grid) -> ConwaySim {
        ConwaySim { grid, generation: 0 }
    }

    pub fn get_generation(&self) -> u32 {
        self.generation
    }

    pub fn is_cell_alive(&self, row: u32, col: u32) -> bool {
        self.grid.get(row, col) == Cell::Alive
    }

    pub fn is_any_cell_alive(&self) -> bool {
        let mut alive = false;

        for &cell in self.grid.grid.iter() {
            if cell == Cell::Alive {
                alive = true;
                break;
            }
        }
        
        return alive;
    }

    pub fn get_neighbor_count(&self, row: u32, col: u32) -> u8 {
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
        if (row > 0) && ((col + 1) < self.grid.num_cols) {
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
        if (col + 1) < self.grid.num_cols {
            new_row = row;
            new_col = col + 1;

            if self.is_cell_alive(new_row, new_col) { count += 1; }
        }

        // check bottom left neighbor
        if ((row + 1) < self.grid.num_rows) && (col > 0) {
            new_row = row + 1;
            new_col = col - 1;

            if self.is_cell_alive(new_row, new_col) { count += 1; }
        }

        // check bottom center neighbor
        if (row + 1) < self.grid.num_rows {
            new_row = row + 1;
            new_col = col;

            if self.is_cell_alive(new_row, new_col) { count += 1; }
        }

        // check bottom left neighbor
        if ((row + 1) < self.grid.num_rows)
                && ((col + 1) < self.grid.num_cols) {
            new_row = row + 1;
            new_col = col + 1;

            if self.is_cell_alive(new_row, new_col) { count += 1; }
        }

        return count;
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        self.grid.set_cells(cells);
    }

    fn apply_rules(&self, row: u32, col: u32) -> Vec<Operation> {
        let mut operations: Vec<Operation> = Vec::new();

        // determine the number of live neighbors to the current cell
        let neighbor_count = self.get_neighbor_count(row, col);

        // determine if the current cell is alive
        let alive = self.is_cell_alive(row, col);

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

    pub fn step(&mut self) {
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

impl fmt::Display for ConwaySim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.grid.fmt(f)
    }
}

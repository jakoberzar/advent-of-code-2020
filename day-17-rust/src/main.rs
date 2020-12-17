use std::collections::VecDeque;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-17.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-17.txt");

const MAX_ITERATIONS: usize = 6;

// TODO: This day could have been done much much nicer if const generics were available on stable.
// It's kind of redundant to work on this at this moment as it's possible that it will be easier
// to refactor in a few weeks time.
// It is quite fast though.
fn main() {
    // Star 1
    let mut grid3d = Grid3D::new(INPUT);
    let active_count = star1(&mut grid3d);
    println!("There are {} cubes active after boot cycle.", active_count);

    // Star 2
    let mut grid4d = Grid4D::new(INPUT);
    let active_count = star2(&mut grid4d);
    println!(
        "There are {} cubes active in 4D after boot cycle.",
        active_count
    );
}

fn star1(grid: &mut Grid3D) -> usize {
    grid.boot_cycle();
    grid.count_active()
}

fn star2(grid: &mut Grid4D) -> usize {
    grid.boot_cycle();
    grid.count_active()
}

type Coords = (usize, usize, usize);

struct Grid3D {
    active_grid: Vec<bool>,
    neighbor_count_grid: Vec<u8>,
    active_list: VecDeque<Coords>,
    max_dimensions: Coords,
    current_min: Coords,
    current_max: Coords,
}

impl Grid3D {
    fn new(input: &str) -> Grid3D {
        let lines = input.trim().lines();

        let mut dim_count = lines.clone();
        let x_dim = dim_count.next().unwrap().len();
        let y_dim = dim_count.count() + 1;
        let max_dimensions = (
            x_dim + 2 * MAX_ITERATIONS,
            y_dim + 2 * MAX_ITERATIONS,
            1 + 2 * MAX_ITERATIONS,
        );

        let current_min = (MAX_ITERATIONS, MAX_ITERATIONS, MAX_ITERATIONS);
        let current_max = (
            max_dimensions.0 - MAX_ITERATIONS,
            max_dimensions.1 - MAX_ITERATIONS,
            max_dimensions.2 - MAX_ITERATIONS,
        );

        let max_len = max_dimensions.0 * max_dimensions.1 * max_dimensions.2;

        let mut active_grid = vec![false; max_len];
        let neighbor_count_grid = vec![0; max_len];
        let mut active_list = VecDeque::new();

        let z_idx = MAX_ITERATIONS;
        for (row_idx, line) in lines.enumerate() {
            let y_idx = MAX_ITERATIONS + row_idx;
            for (col_idx, letter) in line.trim().chars().enumerate() {
                let x_idx = MAX_ITERATIONS + col_idx;
                let idx = x_idx
                    + y_idx * (max_dimensions.0)
                    + z_idx * (max_dimensions.0 * max_dimensions.1);
                let active = letter == '#';
                active_grid[idx] = active;
                if active {
                    active_list.push_back((x_idx, y_idx, z_idx));
                }
            }
        }

        Grid3D {
            active_grid,
            neighbor_count_grid,
            active_list,
            max_dimensions,
            current_min,
            current_max,
        }
    }

    fn get_idx(&self, coords: &Coords) -> usize {
        coords.0
            + coords.1 * (self.max_dimensions.0)
            + coords.2 * (self.max_dimensions.0 * self.max_dimensions.1)
    }

    fn mark_neighbors(&mut self) {
        while let Some(active_cube) = self.active_list.pop_front() {
            // Mark all neighbors
            for z_idx in (active_cube.2 - 1)..=(active_cube.2 + 1) {
                for y_idx in (active_cube.1 - 1)..=(active_cube.1 + 1) {
                    for x_idx in (active_cube.0 - 1)..=(active_cube.0 + 1) {
                        let coords = (x_idx, y_idx, z_idx);
                        if coords == active_cube {
                            continue;
                        }
                        let idx = self.get_idx(&coords);
                        self.neighbor_count_grid[idx] += 1;
                    }
                }
            }
        }
    }

    fn update_active(&mut self) {
        // Check for new active
        for z_idx in (self.current_min.2)..(self.current_max.2) {
            for y_idx in (self.current_min.1)..(self.current_max.1) {
                for x_idx in (self.current_min.0)..(self.current_max.0) {
                    let coords = (x_idx, y_idx, z_idx);
                    let idx = self.get_idx(&coords);
                    let neighbor_count = self.neighbor_count_grid[idx];
                    let is_active = self.active_grid[idx];
                    self.neighbor_count_grid[idx] = 0;
                    if (is_active && (neighbor_count == 2 || neighbor_count == 3))
                        || (!is_active && neighbor_count == 3)
                    {
                        // Becomes / stays active
                        self.active_grid[idx] = true;
                        self.active_list.push_back(coords);
                    } else {
                        // Becomes / stays inactive
                        self.active_grid[idx] = false;
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.mark_neighbors();

        // Expand for one iteration
        self.current_min = (
            self.current_min.0 - 1,
            self.current_min.1 - 1,
            self.current_min.2 - 1,
        );
        self.current_max = (
            self.current_max.0 + 1,
            self.current_max.1 + 1,
            self.current_max.2 + 1,
        );

        // Update the actives
        self.update_active();
    }

    fn boot_cycle(&mut self) {
        for _ in 0..MAX_ITERATIONS {
            self.cycle();
        }
    }

    fn count_active(&self) -> usize {
        self.active_list.len()
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        println!("Printing grid:");
        for z_idx in (self.current_min.2)..(self.current_max.2) {
            println!("Printing z_idx={}", z_idx as i64 - MAX_ITERATIONS as i64);
            for y_idx in (self.current_min.1)..(self.current_max.1) {
                for x_idx in (self.current_min.0)..(self.current_max.0) {
                    let coords = (x_idx, y_idx, z_idx);
                    let idx = self.get_idx(&coords);
                    let is_active = self.active_grid[idx];
                    if is_active {
                        print!("#");
                    } else {
                        print!(".")
                    }
                }
                println!();
            }
        }
        println!();
    }
}

type Coords4D = (usize, usize, usize, usize);

struct Grid4D {
    active_grid: Vec<bool>,
    neighbor_count_grid: Vec<u8>,
    active_list: VecDeque<Coords4D>,
    max_dimensions: Coords4D,
    current_min: Coords4D,
    current_max: Coords4D,
}

impl Grid4D {
    fn new(input: &str) -> Grid4D {
        let lines = input.trim().lines();

        let mut dim_count = lines.clone();
        let x_dim = dim_count.next().unwrap().len();
        let y_dim = dim_count.count() + 1;
        let max_dimensions = (
            x_dim + 2 * MAX_ITERATIONS,
            y_dim + 2 * MAX_ITERATIONS,
            1 + 2 * MAX_ITERATIONS,
            1 + 2 * MAX_ITERATIONS,
        );

        let current_min = (
            MAX_ITERATIONS,
            MAX_ITERATIONS,
            MAX_ITERATIONS,
            MAX_ITERATIONS,
        );
        let current_max = (
            max_dimensions.0 - MAX_ITERATIONS,
            max_dimensions.1 - MAX_ITERATIONS,
            max_dimensions.2 - MAX_ITERATIONS,
            max_dimensions.3 - MAX_ITERATIONS,
        );

        let max_len = max_dimensions.0 * max_dimensions.1 * max_dimensions.2 * max_dimensions.3;

        let mut active_grid = vec![false; max_len];
        let neighbor_count_grid = vec![0; max_len];
        let mut active_list = VecDeque::new();

        let w_idx = MAX_ITERATIONS;
        let z_idx = MAX_ITERATIONS;
        for (row_idx, line) in lines.enumerate() {
            let y_idx = MAX_ITERATIONS + row_idx;
            for (col_idx, letter) in line.trim().chars().enumerate() {
                let x_idx = MAX_ITERATIONS + col_idx;
                let idx = x_idx
                    + y_idx * (max_dimensions.0)
                    + z_idx * (max_dimensions.0 * max_dimensions.1)
                    + w_idx * (max_dimensions.0 * max_dimensions.1 * max_dimensions.2);
                let active = letter == '#';
                active_grid[idx] = active;
                if active {
                    active_list.push_back((x_idx, y_idx, z_idx, w_idx));
                }
            }
        }

        Grid4D {
            active_grid,
            neighbor_count_grid,
            active_list,
            max_dimensions,
            current_min,
            current_max,
        }
    }

    fn get_idx(&self, coords: &Coords4D) -> usize {
        coords.0
            + coords.1 * (self.max_dimensions.0)
            + coords.2 * (self.max_dimensions.0 * self.max_dimensions.1)
            + coords.3 * (self.max_dimensions.0 * self.max_dimensions.1 * self.max_dimensions.2)
    }

    fn mark_neighbors(&mut self) {
        while let Some(active_cube) = self.active_list.pop_front() {
            // Mark all neighbors
            for w_idx in (active_cube.3 - 1)..=(active_cube.3 + 1) {
                for z_idx in (active_cube.2 - 1)..=(active_cube.2 + 1) {
                    for y_idx in (active_cube.1 - 1)..=(active_cube.1 + 1) {
                        for x_idx in (active_cube.0 - 1)..=(active_cube.0 + 1) {
                            let coords = (x_idx, y_idx, z_idx, w_idx);
                            if coords == active_cube {
                                continue;
                            }
                            let idx = self.get_idx(&coords);
                            self.neighbor_count_grid[idx] += 1;
                        }
                    }
                }
            }
        }
    }

    fn update_active(&mut self) {
        // Check for new active
        for w_idx in (self.current_min.3)..(self.current_max.3) {
            for z_idx in (self.current_min.2)..(self.current_max.2) {
                for y_idx in (self.current_min.1)..(self.current_max.1) {
                    for x_idx in (self.current_min.0)..(self.current_max.0) {
                        let coords = (x_idx, y_idx, z_idx, w_idx);
                        let idx = self.get_idx(&coords);
                        let neighbor_count = self.neighbor_count_grid[idx];
                        let is_active = self.active_grid[idx];
                        self.neighbor_count_grid[idx] = 0;
                        if (is_active && (neighbor_count == 2 || neighbor_count == 3))
                            || (!is_active && neighbor_count == 3)
                        {
                            // Becomes / stays active
                            self.active_grid[idx] = true;
                            self.active_list.push_back(coords);
                        } else {
                            // Becomes / stays inactive
                            self.active_grid[idx] = false;
                        }
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.mark_neighbors();

        // Expand for one iteration
        self.current_min = (
            self.current_min.0 - 1,
            self.current_min.1 - 1,
            self.current_min.2 - 1,
            self.current_min.3 - 1,
        );
        self.current_max = (
            self.current_max.0 + 1,
            self.current_max.1 + 1,
            self.current_max.2 + 1,
            self.current_max.3 + 1,
        );

        // Update the actives
        self.update_active();
    }

    fn boot_cycle(&mut self) {
        for _ in 0..MAX_ITERATIONS {
            self.cycle();
        }
    }

    fn count_active(&self) -> usize {
        self.active_list.len()
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        println!("Printing grid:");
        for w_idx in (self.current_min.3)..(self.current_max.3) {
            for z_idx in (self.current_min.2)..(self.current_max.2) {
                println!(
                    "Printing w_idx={}, z_idx={}",
                    w_idx as i64 - MAX_ITERATIONS as i64,
                    z_idx as i64 - MAX_ITERATIONS as i64
                );
                for y_idx in (self.current_min.1)..(self.current_max.1) {
                    for x_idx in (self.current_min.0)..(self.current_max.0) {
                        let coords = (x_idx, y_idx, z_idx, 1);
                        let idx = self.get_idx(&coords);
                        let is_active = self.active_grid[idx];
                        if is_active {
                            print!("#");
                        } else {
                            print!(".")
                        }
                    }
                    println!();
                }
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let mut grid = Grid3D::new(SIMPLE_INPUT);
        assert_eq!(star1(&mut grid), 112);
    }

    #[test]
    fn full_star1() {
        let mut grid = Grid3D::new(INPUT);
        assert_eq!(star1(&mut grid), 289);
    }

    #[test]
    fn simple_star2() {
        let mut grid = Grid4D::new(SIMPLE_INPUT);
        assert_eq!(star2(&mut grid), 848);
    }

    #[test]
    fn full_star2() {
        let mut grid = Grid4D::new(INPUT);
        assert_eq!(star2(&mut grid), 2084);
    }
}

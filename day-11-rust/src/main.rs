use std::{convert::TryInto, fmt};

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-11.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-11.txt");

fn main() {
    let mut input = parse_input(INPUT);
    let mut input_star2 = input.clone();

    // Star 1
    let seat_count = star1(&mut input);
    println!("{} seats are occupied.", seat_count);

    // Star 2
    let seat_count = star2(&mut input_star2);
    println!("{} seats are occupied using new rules.", seat_count);
}

fn parse_input(input: &str) -> Simulator {
    Simulator::new(input)
}

fn star1(simulator: &mut Simulator) -> usize {
    simulator.simulate_neighbors()
}

fn star2(simulator: &mut Simulator) -> usize {
    simulator.simulate_first_seats()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PositionStatus {
    Floor,
    EmptySeat,
    TakenSeat,
}

impl fmt::Display for PositionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionStatus::Floor => write!(f, "."),
            PositionStatus::EmptySeat => write!(f, "L"),
            PositionStatus::TakenSeat => write!(f, "#"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Grid {
    data: Vec<PositionStatus>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let lines = input.trim().lines();
        let mut size_iterator = lines.clone();
        let width = size_iterator.next().unwrap().trim().len();
        let height = 1 + size_iterator.count();
        let grid: Vec<PositionStatus> = lines
            .map(|line| {
                line.chars().map(|mark| match mark {
                    '.' => PositionStatus::Floor,
                    'L' => PositionStatus::EmptySeat,
                    '#' => PositionStatus::TakenSeat,
                    _ => panic!("Invalid character!"),
                })
            })
            .flatten()
            .collect();

        Grid {
            data: grid,
            width,
            height,
        }
    }

    fn at(&self, row: usize, col: usize) -> PositionStatus {
        let idx = row * self.width + col;
        self.data[idx]
    }

    fn edit(&mut self, row: usize, col: usize, status: PositionStatus) {
        let idx = row * self.width + col;
        self.data[idx] = status;
    }

    // TODO: We could have also just added padding of floor around... :)
    fn count_occupied_neighbors(&self, row: usize, col: usize) -> u32 {
        let mut sum = 0;
        let occupied = PositionStatus::TakenSeat;
        if row > 0 {
            // Top-middle
            if self.at(row - 1, col) == occupied {
                sum += 1;
            }
            // Top-left
            if col > 0 && self.at(row - 1, col - 1) == occupied {
                sum += 1;
            }
            // Top-right
            if col < self.width - 1 && self.at(row - 1, col + 1) == occupied {
                sum += 1;
            }
        }
        if row < self.height - 1 {
            // Bottom-middle
            if self.at(row + 1, col) == occupied {
                sum += 1;
            }
            // Bottom-left
            if col > 0 && self.at(row + 1, col - 1) == occupied {
                sum += 1;
            }
            // Bottom-right
            if col < self.width - 1 && self.at(row + 1, col + 1) == occupied {
                sum += 1;
            }
        }
        // Mid-left
        if col > 0 && self.at(row, col - 1) == occupied {
            sum += 1;
        }
        // Mid-right
        if col < self.width - 1 && self.at(row, col + 1) == occupied {
            sum += 1;
        }

        sum
    }

    fn find_first_seat(
        &self,
        start_row: usize,
        start_col: usize,
        update_row: &dyn Fn(i64) -> i64,
        update_col: &dyn Fn(i64) -> i64,
    ) -> PositionStatus {
        let height: i64 = self.height.try_into().unwrap();
        let width: i64 = self.width.try_into().unwrap();
        let mut row: i64 = update_row(start_row.try_into().unwrap());
        let mut col: i64 = update_col(start_col.try_into().unwrap());
        // Skip floor
        loop {
            // Check bounds
            if row < 0 || col < 0 || row == height || col == width {
                return PositionStatus::Floor;
            }
            // Check if seat found
            let seat = self.at(row as usize, col as usize); // Bounds check is already done above
            if seat != PositionStatus::Floor {
                return seat;
            }
            // Update position
            row = update_row(row);
            col = update_col(col);
        }
    }

    fn count_occupied_first_seats(&self, row: usize, col: usize) -> u32 {
        let directions: [PositionStatus; 8] = [
            self.find_first_seat(row, col, &|row| row - 1, &|col| col - 1),
            self.find_first_seat(row, col, &|row| row - 1, &|col| col),
            self.find_first_seat(row, col, &|row| row - 1, &|col| col + 1),
            self.find_first_seat(row, col, &|row| row, &|col| col - 1),
            self.find_first_seat(row, col, &|row| row, &|col| col + 1),
            self.find_first_seat(row, col, &|row| row + 1, &|col| col - 1),
            self.find_first_seat(row, col, &|row| row + 1, &|col| col),
            self.find_first_seat(row, col, &|row| row + 1, &|col| col + 1),
        ];

        directions
            .iter()
            .filter(|seat| **seat == PositionStatus::TakenSeat)
            .count() as u32 // Always <= 8
    }

    fn count_occupied_seats(&self) -> usize {
        self.data
            .iter()
            .filter(|seat| **seat == PositionStatus::TakenSeat)
            .count()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.data[y * self.width + x]).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Simulator {
    grid1: Grid,
    grid2: Grid,
    current_grid: u32,
    width: usize,
    height: usize,
}

impl Simulator {
    fn new(input: &str) -> Simulator {
        let grid = Grid::new(input);

        Simulator {
            width: grid.width,
            height: grid.height,
            grid1: grid.clone(),
            grid2: grid,
            current_grid: 1,
        }
    }

    fn current_grid_ref(&self) -> &Grid {
        if self.current_grid == 1 {
            &self.grid1
        } else {
            &self.grid2
        }
    }

    fn next_grid_ref(&self) -> &Grid {
        if self.current_grid == 1 {
            &self.grid2
        } else {
            &self.grid1
        }
    }

    fn next_grid_mut_ref(&mut self) -> &mut Grid {
        if self.current_grid == 1 {
            &mut self.grid2
        } else {
            &mut self.grid1
        }
    }

    fn iterate_neighbors(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let current_state = self.current_grid_ref().at(row, col);
                if current_state == PositionStatus::Floor {
                    // Floor shouldn't ever change; currently it is default for empty
                    continue;
                };
                let occupied_neighbors = self.current_grid_ref().count_occupied_neighbors(row, col);
                let next_status = match (current_state, occupied_neighbors) {
                    (PositionStatus::EmptySeat, 0) => PositionStatus::TakenSeat,
                    (PositionStatus::TakenSeat, x) if x >= 4 => PositionStatus::EmptySeat,
                    _ => current_state,
                };
                self.next_grid_mut_ref().edit(row, col, next_status);
            }
        }
        self.current_grid = if self.current_grid == 1 { 2 } else { 1 };
    }

    fn iterate_first_seat(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let current_state = self.current_grid_ref().at(row, col);
                if current_state == PositionStatus::Floor {
                    // Floor shouldn't ever change; currently it is default for empty
                    continue;
                };
                let occupied_neighbors =
                    self.current_grid_ref().count_occupied_first_seats(row, col);
                let next_status = match (current_state, occupied_neighbors) {
                    (PositionStatus::EmptySeat, 0) => PositionStatus::TakenSeat,
                    (PositionStatus::TakenSeat, x) if x >= 5 => PositionStatus::EmptySeat,
                    _ => current_state,
                };
                self.next_grid_mut_ref().edit(row, col, next_status);
            }
        }
        self.current_grid = if self.current_grid == 1 { 2 } else { 1 };
    }

    fn simulate_neighbors(&mut self) -> usize {
        let mut iterations = 0;
        while iterations == 0 || self.grid1 != self.grid2 {
            self.iterate_neighbors();
            iterations += 1;
        }
        self.current_grid_ref().count_occupied_seats()
    }

    fn simulate_first_seats(&mut self) -> usize {
        let mut iterations = 0;
        while iterations == 0 || self.grid1 != self.grid2 {
            self.iterate_first_seat();
            iterations += 1;
        }
        self.current_grid_ref().count_occupied_seats()
    }
}

impl fmt::Display for Simulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Current grid:\n").unwrap();
        write!(f, "{}", self.current_grid_ref()).unwrap();
        write!(f, "\n").unwrap();
        write!(f, "Next grid:\n").unwrap();
        write!(f, "{}", self.next_grid_ref()).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let mut simulator = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&mut simulator), 37);
    }

    #[test]
    fn full_star1() {
        let mut simulator = parse_input(INPUT);
        assert_eq!(star1(&mut simulator), 2470);
    }

    #[test]
    fn simple_star2() {
        let mut simulator = parse_input(SIMPLE_INPUT);
        assert_eq!(star2(&mut simulator), 26);
    }

    #[test]
    fn full_star2() {
        let mut simulator = parse_input(INPUT);
        assert_eq!(star2(&mut simulator), 2259);
    }
}

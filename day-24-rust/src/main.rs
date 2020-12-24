use std::{collections::HashSet, convert::TryFrom};

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-24.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-24.txt");

const NEIGHBOUR_DIFFS: [(i64, i64); 6] = [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)];

fn main() {
    let paths = parse_input(INPUT);

    // Star 1
    let initial_black_tiles = star1(&paths);
    println!(
        "After initialization, there are {} black tiles",
        initial_black_tiles
    );

    // Star 2
    let day_100_black_tiles = star2(&paths);
    println!(
        "After 100 days, there are {} black tiles",
        day_100_black_tiles
    );
}

fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            let mut directions: Vec<Direction> = Vec::new();
            let mut consumed = 0;
            while consumed < line.len() {
                let (direction, chars) = Direction::from_larger_string(&line[consumed..]).unwrap();
                directions.push(direction);
                consumed += chars;
            }
            directions
        })
        .collect()
}

fn star1(paths: &Vec<Vec<Direction>>) -> usize {
    let max_possible_size = paths.iter().map(|path| path.len()).max().unwrap();
    let mut grid = HexGrid::new(max_possible_size);
    grid.init_from_paths(paths);
    grid.count_black()
}

fn star2(paths: &Vec<Vec<Direction>>) -> usize {
    let max_possible_size = paths.iter().map(|path| path.len()).max().unwrap();
    let amount_of_days = 100;
    let mut grid = HexGrid::new(max_possible_size + amount_of_days);
    grid.init_from_paths(paths);
    for _day in 1..=amount_of_days {
        grid.next_day();
    }
    grid.count_black()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    // Return the direction and the amount of characters consumed
    fn from_larger_string(input: &str) -> Option<(Direction, usize)> {
        let dir_two_chars = if input.len() >= 2 {
            match &input[0..2] {
                "se" => Some(Direction::SouthEast),
                "sw" => Some(Direction::SouthWest),
                "ne" => Some(Direction::NorthEast),
                "nw" => Some(Direction::NorthWest),
                _ => None,
            }
        } else {
            None
        };

        if let Some(dir) = dir_two_chars {
            Some((dir, 2))
        } else if input.len() == 0 {
            None
        } else {
            match &input[0..1] {
                "e" => Some((Direction::East, 1)),
                "w" => Some((Direction::West, 1)),
                _ => None,
            }
        }
    }

    fn to_diff(&self) -> (i64, i64) {
        match self {
            Direction::East => (1, 0),
            Direction::SouthEast => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (0, -1),
            Direction::NorthEast => (1, -1),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Color {
    Black,
    White,
}

impl Color {
    fn opposite(self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Coords {
    r: usize,
    q: usize,
}

impl Coords {
    fn new(q: usize, r: usize) -> Self {
        Self { q, r }
    }
    fn with_diff(&self, (q, r): &(i64, i64)) -> Self {
        let abs_q = i64::try_from(self.q).unwrap() + q;
        let abs_r = i64::try_from(self.r).unwrap() + r;
        Coords {
            q: usize::try_from(abs_q).unwrap(),
            r: usize::try_from(abs_r).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct HexGrid {
    // using the axial coordinate system
    grid: Vec<Color>,
    side_len: usize,
    center: Coords,
    black_list: HashSet<Coords>,
}

impl HexGrid {
    fn new(max_size: usize) -> HexGrid {
        let side_len = max_size * 2 + 1;
        HexGrid {
            grid: vec![Color::White; side_len * side_len],
            side_len,
            center: Coords::new(max_size, max_size),
            black_list: HashSet::new(),
        }
    }

    fn at(&self, pos: &Coords) -> Color {
        self.grid[pos.r * self.side_len + pos.q]
    }

    fn change_at(&mut self, pos: &Coords, color: Color) {
        self.grid[pos.r * self.side_len + pos.q] = color;
        if color == Color::Black {
            self.black_list.insert(*pos);
        } else {
            self.black_list.remove(pos);
        }
    }

    fn init_from_paths(&mut self, paths: &Vec<Vec<Direction>>) {
        for path in paths {
            self.flip_tile(path);
        }
    }

    fn flip_tile(&mut self, path: &Vec<Direction>) {
        let pos = path
            .iter()
            .fold(self.center, |acc, x| acc.with_diff(&x.to_diff()));
        let color = self.at(&pos);
        self.change_at(&pos, color.opposite());
    }

    fn count_black(&self) -> usize {
        self.black_list.len()
    }

    fn count_black_neighbours(&self, position: &Coords) -> usize {
        let mut count = 0;
        for diff in NEIGHBOUR_DIFFS.iter() {
            let pos = position.with_diff(diff);
            let color = self.at(&pos);
            if color == Color::Black {
                count += 1;
            }
        }
        count
    }

    fn next_day(&mut self) {
        let mut possible_changes = Vec::with_capacity(self.black_list.len() * 7);
        for pos in self.black_list.iter() {
            possible_changes.push(*pos);
            for diff in NEIGHBOUR_DIFFS.iter() {
                possible_changes.push(pos.with_diff(diff));
            }
        }

        let mut flip_list: Vec<(Coords, Color)> = Vec::new();

        for pos in possible_changes.iter() {
            let black = self.count_black_neighbours(pos);
            let color = self.at(pos);
            if color == Color::Black && (black == 0 || black > 2) {
                flip_list.push((*pos, Color::White));
            } else if color == Color::White && black == 2 {
                flip_list.push((*pos, Color::Black));
            }
        }

        for (pos, color) in flip_list {
            self.change_at(&pos, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn very_simple() {
        let paths = parse_input("esew\nnwwswee");
        assert_eq!(star1(&paths), 2);
    }

    #[test]
    fn simple_star1() {
        let paths = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&paths), 10);
    }

    #[test]
    fn full_star1() {
        let paths = parse_input(INPUT);
        assert_eq!(star1(&paths), 549);
    }

    #[test]
    fn simple_star2() {
        let paths = parse_input(SIMPLE_INPUT);
        assert_eq!(star2(&paths), 2208);
    }

    #[test]
    fn full_star2() {
        let paths = parse_input(INPUT);
        assert_eq!(star2(&paths), 4147);
    }
}

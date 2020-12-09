use std::fmt;

#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-03.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-03.txt");

fn main() {
    let map = parse_input(INPUT);

    // Star 1
    let tree_count = star1(&map);
    println!(
        "Encountered {} trees on slope right {}, down {}.",
        tree_count, 3, 1
    );

    // Star 2
    let multiplied = star2(&map);
    println!("Multiplied number of trees is {}.", multiplied);
}

fn star1(map: &Map) -> usize {
    encountered_trees(&map, 3, 1)
}

fn star2(map: &Map) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| encountered_trees(map, *right, *down))
        .product()
}

fn parse_input(input: &str) -> Map {
    Map::new(input)
}

fn encountered_trees(map: &Map, right: usize, down: usize) -> usize {
    let mut tree_count = 0;
    let mut y = down;
    let mut x = right;
    while y < map.height {
        if let Field::Tree = map.at(x, y) {
            tree_count += 1;
        }
        y += down;
        x += right;
    }
    tree_count
}

#[allow(dead_code)] // Allow visited options for debugging
#[derive(Copy, Clone, Debug)]
enum Field {
    Open,
    Tree,
    VisitedOpen,
    VisitedTree,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Open => write!(f, "."),
            Field::Tree => write!(f, "#"),
            Field::VisitedOpen => write!(f, "O"),
            Field::VisitedTree => write!(f, "X"),
        }
    }
}

#[derive(Debug)]
struct Map {
    data: Vec<Field>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Map {
        let lines = input.trim().lines();
        let width = lines.clone().next().expect("Empty input!").len();
        let height = lines.clone().count();
        let data: Vec<Field> = lines
            .flat_map(|line| {
                line.trim()
                    .chars()
                    .map(|ch| if ch == '.' { Field::Open } else { Field::Tree })
            })
            .collect();

        Map {
            data,
            width,
            height,
        }
    }

    fn at(&self, x: usize, y: usize) -> Field {
        let finite_x = x % self.width;

        self.data[y * self.width + finite_x]
    }

    #[allow(dead_code)] // Allow option to visit the map for debugging
    fn visit(&mut self, x: usize, y: usize) {
        let finite_x = x % self.width;
        let idx = y * self.width + finite_x;
        self.data[idx] = match self.data[idx] {
            Field::Open => Field::VisitedOpen,
            Field::Tree => Field::VisitedTree,
            _ => panic!("Visited ({}, {}) twice!", finite_x, y),
        }
    }
}

impl fmt::Display for Map {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_star1() {
        let map = parse_input(INPUT);
        assert_eq!(star1(&map), 278);
    }

    #[test]
    fn full_star2() {
        let map = parse_input(INPUT);
        assert_eq!(star2(&map), 9709761600);
    }
}

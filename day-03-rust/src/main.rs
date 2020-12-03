use std::fmt;

#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-03.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-03.txt");

fn main() {
    let map = parse_input(INPUT);
    star1(&map);
    star2(&map);
}

fn star1(map: &Map) {
    let tree_count = encountered_trees(&map, 3, 1);
    println!(
        "Encountered {} trees on slope right {}, down {}.",
        tree_count, 3, 1
    );
    assert_eq!(278, tree_count);
}

fn star2(map: &Map) {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let trees = slopes
        .iter()
        .map(|(right, down)| encountered_trees(map, *right, *down));
    let multiplied = trees.fold(1, |acc, x| acc * x);

    println!("Multiplied number of trees is {}.", multiplied);
    assert_eq!(9709761600, multiplied);
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

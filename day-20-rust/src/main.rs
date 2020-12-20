use std::collections::HashMap;
use std::fmt;

const MONSTER: &str = include_str!("./../../inputs/day-20-monster.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-20.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-20.txt");

fn main() {
    let mut tiles = parse_input(INPUT);

    // Star 1
    star1(&mut tiles);
}

fn parse_input(input: &str) -> Vec<Tile> {
    input.trim().split("\n\n").map(Tile::new).collect()
}

fn star1(tiles: &mut [Tile]) -> u64 {
    let mut matcher = TileMatcher::new(tiles);
    matcher.find_matches();
    let tiles_per_matches = matcher.get_tiles_per_matches();
    tiles_per_matches[2]
        .iter()
        .map(|idx| tiles[*idx].id as u64)
        .product()
}

fn star2(tiles: &mut [Tile]) -> usize {
    let mut matcher = TileMatcher::new(tiles);
    matcher.find_matches();
    let mut picture = matcher.construct_picture();
    picture.rotate_and_flip_until_monster_found();
    println!("{}", picture);
    picture.mark_monsters();
    println!("{}", picture);
    picture.count_roughness()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    fn rotate_right(self) -> Self {
        let idx: u8 = self.into();
        Self::from((idx + 1) % 4)
    }

    fn opposite(self) -> Self {
        let idx: u8 = self.into();
        Self::from((idx + 2) % 4)
    }
}

impl From<u8> for Side {
    fn from(idx: u8) -> Self {
        match idx {
            0 => Side::Top,
            1 => Side::Right,
            2 => Side::Bottom,
            3 => Side::Left,
            _ => panic!("Invalid index!"),
        }
    }
}

impl Into<u8> for Side {
    fn into(self) -> u8 {
        match self {
            Side::Top => 0,
            Side::Right => 1,
            Side::Bottom => 2,
            Side::Left => 3,
            _ => panic!("Invalid index!"),
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    grid: Vec<char>,
    borders: [Vec<char>; 4],         // top, right, bottom, left
    borders_flipped: [Vec<char>; 4], // top, right, bottom, left
    line_len: usize,
}

impl Tile {
    fn new(input: &str) -> Tile {
        let mut lines = input.trim().lines();

        let id = lines
            .next()
            .unwrap()
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse()
            .unwrap();

        let borders: [Vec<char>; 4] = [vec![], vec![], vec![], vec![]];
        let borders_flipped: [Vec<char>; 4] = [vec![], vec![], vec![], vec![]];
        let line_len = lines.clone().next().unwrap().trim().len();
        let grid = lines.flat_map(|line| line.trim().chars()).collect();

        let mut tile = Tile {
            id,
            grid,
            borders,
            borders_flipped,
            line_len,
        };

        tile.update_borders();
        tile
    }

    fn update_borders(&mut self) {
        let top_border = self.grid[0..self.line_len].to_owned();
        let bottom_border = self.grid[self.grid.len() - self.line_len..self.grid.len()].to_owned();
        let left_border: Vec<char> = self.grid.iter().step_by(self.line_len).copied().collect();
        let right_border = self
            .grid
            .iter()
            .skip(self.line_len - 1)
            .step_by(self.line_len)
            .copied()
            .collect();
        self.borders = [top_border, right_border, bottom_border, left_border];

        self.borders_flipped = self.borders.clone();
        for border in self.borders_flipped.iter_mut() {
            border.reverse();
        }
    }

    fn find_match(&self, other: &Tile) -> Option<TileMatch> {
        let id1 = self.id;
        let id2 = other.id;
        for (idx1, border) in self.borders.iter().enumerate() {
            let found = other.borders.iter().position(|other| other == border);
            if let Some(idx2) = found {
                return Some(TileMatch::new(id1, id2, idx1));
            }

            // Now try with borders flipped
            let found = other
                .borders_flipped
                .iter()
                .position(|other| other == border);
            if let Some(idx2) = found {
                return Some(TileMatch::new(id1, id2, idx1));
            }
        }
        None
    }

    fn rotate_right(&mut self) {
        let mut new_grid: Vec<char> = Vec::with_capacity(self.grid.len());
        for new_idx in 0..self.grid.len() {
            let new_row = new_idx / self.line_len;
            let new_col = new_idx % self.line_len;
            let old_row = (self.line_len - 1) - new_col;
            let old_col = new_row;
            let value = self.grid[old_row * self.line_len + old_col];
            new_grid.push(value);
        }
        self.grid = new_grid;
        self.update_borders();
    }

    fn flip_horizontally(&mut self) {
        let mut new_grid: Vec<char> = Vec::with_capacity(self.grid.len());
        for new_idx in 0..self.grid.len() {
            let col = new_idx % self.line_len;
            let new_row = new_idx / self.line_len;
            let old_row = (self.line_len - 1) - new_row;
            let value = self.grid[old_row * self.line_len + col];
            new_grid.push(value);
        }
        self.grid = new_grid;
        self.update_borders();
    }

    fn flip_vertically(&mut self) {
        let mut new_grid: Vec<char> = Vec::with_capacity(self.grid.len());
        for new_idx in 0..self.grid.len() {
            let row = new_idx / self.line_len;
            let new_col = new_idx % self.line_len;
            let old_col = (self.line_len - 1) - new_col;
            let value = self.grid[row * self.line_len + old_col];
            new_grid.push(value);
        }
        self.grid = new_grid;
        self.update_borders();
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.line_len {
            for x in 0..self.line_len {
                write!(f, "{}", self.grid[y * self.line_len + x]).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct TileMatch {
    id1: u32,
    id2: u32,
    border1: Side,
}

impl TileMatch {
    fn new(id1: u32, id2: u32, idx1: usize) -> TileMatch {
        TileMatch {
            id1,
            id2,
            border1: Side::from(idx1 as u8),
        }
    }
}

struct TileMatcher<'a> {
    tiles: &'a mut [Tile],
    tile_match: Vec<Vec<TileMatch>>,
    id_idx_map: HashMap<u32, usize>,
}

impl<'a> TileMatcher<'a> {
    fn new(tiles: &mut [Tile]) -> TileMatcher {
        let id_idx_map = tiles
            .iter()
            .enumerate()
            .map(|(idx, tile)| (tile.id, idx))
            .collect();
        let tile_match = vec![Vec::new(); tiles.len()];
        TileMatcher {
            tiles,
            tile_match,
            id_idx_map,
        }
    }

    fn find_matches(&mut self) {
        for (idx1, tile1) in self.tiles.iter().enumerate() {
            for (idx2, tile2) in self.tiles.iter().enumerate() {
                if idx1 == idx2 {
                    continue;
                }

                let matched = tile1.find_match(tile2);
                if let Some(tile_match) = matched {
                    self.tile_match[idx1].push(tile_match);
                    // TODO: Add to tile_match of second idx as well
                }
            }
        }

        // println!("{:?}", self.tile_match);
    }

    fn get_tiles_per_matches(&self) -> Vec<Vec<usize>> {
        let match_counts: Vec<usize> = self
            .tile_match
            .iter()
            .map(|matches| matches.len())
            .collect();

        let mut partitioned: Vec<Vec<usize>> = vec![];
        for match_amount in 0..=4 {
            partitioned.push(
                match_counts
                    .iter()
                    .enumerate()
                    .filter(|(_, count)| **count == match_amount)
                    .map(|(idx, _)| idx)
                    .collect(),
            );
        }

        assert_eq!(partitioned[2].len(), 4);
        assert_eq!(partitioned[3].len() % 4, 0);
        let side_len = partitioned[2].len() / 2 + partitioned[3].len() / 4;
        assert_eq!(
            partitioned[2].len() + partitioned[3].len() + partitioned[4].len(),
            side_len * side_len
        );

        partitioned
    }

    fn construct_picture(&mut self) -> Picture {
        let tiles_per_matches = self.get_tiles_per_matches();
        let side_len = tiles_per_matches[2].len() / 2 + tiles_per_matches[3].len() / 4;

        // Each field in a grid consists of tile idx, flipped horizontally, flipped vertically, times rotated right
        let mut grid: Vec<Option<usize>> = vec![None; side_len * side_len];

        // Top left tile
        let mut tile_idx = tiles_per_matches[2][0];
        let mut inserted_tile_side: Side;
        let mut inserted_tile_match: TileMatch;
        {
            let matches = &self.tile_match[tile_idx];
            let mut side1 = matches[0].border1;
            let mut side2 = matches[1].border1;
            while !((side1 == Side::Right && side2 == Side::Bottom)
                || (side1 == Side::Bottom && side2 == Side::Right))
            {
                self.rotate_tile_right(tile_idx);
                let matches = &self.tile_match[tile_idx];
                side1 = matches[0].border1;
                side2 = matches[1].border1;
            }
            grid[0] = Some(tile_idx);

            // Store the currently inserted tile
            inserted_tile_side = Side::Right;
            let tm_idx = self.tile_match[tile_idx]
                .iter()
                .position(|tm| tm.border1 == inserted_tile_side)
                .unwrap();
            inserted_tile_match = self.tile_match[tile_idx].remove(tm_idx);
            tile_idx = self.id_idx_map[&inserted_tile_match.id2];
        }

        // Top row
        for col_idx in 1..side_len {
            // First, find the match to the previously inserted tile
            let other_id = inserted_tile_match.id1;
            let tm_idx = self.find_tile_match_idx(tile_idx, other_id);

            // Then, rotate the tile until it fits correctly
            self.rotate_tile_until_tile_match_opposite(tile_idx, tm_idx, inserted_tile_side);

            // Check if it needed to be flipped along the horizon to match
            self.flip_tile_to_match(tile_idx, other_id, inserted_tile_side);

            // Then, insert it into the grid
            grid[col_idx] = Some(tile_idx);

            // Throw current tile out
            self.tile_match[tile_idx].remove(tm_idx);

            // Now, find the next tile. Skip for top right
            if col_idx != side_len - 1 {
                let tm_idx = self.tile_match[tile_idx]
                    .iter()
                    .position(|tm| tm.border1 == inserted_tile_side)
                    .unwrap();
                inserted_tile_match = self.tile_match[tile_idx].remove(tm_idx);
                tile_idx = self.id_idx_map[&inserted_tile_match.id2];
            }
        }

        // Other rows
        for col_idx in 0..side_len {
            inserted_tile_side = Side::Bottom;
            tile_idx = grid[col_idx].unwrap();
            let tm_idx = self.tile_match[tile_idx]
                .iter()
                .position(|tm| tm.border1 == inserted_tile_side)
                .unwrap();
            inserted_tile_match = self.tile_match[tile_idx].remove(tm_idx);
            tile_idx = self.id_idx_map[&inserted_tile_match.id2];

            // Left column
            {
                for row_idx in 1..side_len {
                    // First, find the match to the previously inserted tile
                    let other_id = inserted_tile_match.id1;
                    let tm_idx = self.find_tile_match_idx(tile_idx, other_id);

                    // Then, rotate the tile until it fits correctly
                    self.rotate_tile_until_tile_match_opposite(
                        tile_idx,
                        tm_idx,
                        inserted_tile_side,
                    );

                    // Check if it needed to be flipped along the horizon to match
                    self.flip_tile_to_match(tile_idx, other_id, inserted_tile_side);

                    // Then, insert it into the grid
                    grid[row_idx * side_len + col_idx] = Some(tile_idx);

                    // Throw current tile out
                    self.tile_match[tile_idx].remove(tm_idx);

                    // Rotate if needed
                    if row_idx == side_len - 1 {
                        if col_idx == side_len - 1 {
                            continue;
                        }
                        inserted_tile_side = Side::Right;
                    }
                    // Now, find the next tile
                    let tm_idx = self.tile_match[tile_idx]
                        .iter()
                        .position(|tm| tm.border1 == inserted_tile_side)
                        .unwrap();
                    inserted_tile_match = self.tile_match[tile_idx].remove(tm_idx);
                    tile_idx = self.id_idx_map[&inserted_tile_match.id2];
                }
            }
        }

        self.print_grid(&grid, side_len);

        let final_grid: Vec<usize> = grid.iter().map(|idx| idx.unwrap()).collect();

        Picture::new(self.tiles, &final_grid, side_len)
    }

    fn rotate_tile_right(&mut self, tile_idx: usize) {
        // Rotate the actual tile
        self.tiles[tile_idx].rotate_right();
        // Rotate the tile matches
        for tile_match_idx in 0..self.tile_match[tile_idx].len() {
            let mut tile_match = &mut self.tile_match[tile_idx][tile_match_idx];
            let rotated_border = Side::from(tile_match.border1).rotate_right().into();
            tile_match.border1 = rotated_border;
        }
    }

    fn rotate_tile_until_tile_match_opposite(
        &mut self,
        tile_idx: usize,
        tm_idx: usize,
        opposite_side: Side,
    ) -> u8 {
        let wanted_side = opposite_side.opposite();
        let mut times_rotated = 0;
        while self.tile_match[tile_idx][tm_idx].border1 != wanted_side {
            self.rotate_tile_right(tile_idx);
            times_rotated += 1;
        }
        times_rotated
    }

    fn find_tile_match_idx(&self, tile_idx: usize, other_id: u32) -> usize {
        self.tile_match[tile_idx]
            .iter()
            .position(|tm| tm.id2 == other_id)
            .unwrap()
    }

    fn flip_tile_to_match(&mut self, tile_idx: usize, other_id: u32, other_side: Side) {
        let my_side = other_side.opposite();
        let other_idx = self.id_idx_map[&other_id];
        let other_border: Vec<char> = self.tiles[other_idx].borders[other_side as usize].clone();

        let tile = &mut self.tiles[tile_idx];
        if tile.borders[my_side as usize] != other_border {
            // Try flipping
            match my_side {
                Side::Top | Side::Bottom => tile.flip_vertically(),
                _ => tile.flip_horizontally(),
            }

            // Update other matches
            for tm in self.tile_match[tile_idx].iter_mut() {
                match my_side {
                    Side::Top | Side::Bottom => {
                        if let Side::Left | Side::Right = tm.border1 {
                            tm.border1 = tm.border1.opposite();
                        }
                    }
                    _ => {
                        if let Side::Top | Side::Bottom = tm.border1 {
                            tm.border1 = tm.border1.opposite();
                        }
                    }
                }
            }

            assert!(self.tiles[tile_idx].borders[my_side as usize] == other_border);
        }
    }

    fn print_grid(&self, grid: &[Option<usize>], side_len: usize) {
        let tile_len = self.tiles[0].line_len;
        // ! Only works for one row atm!!!
        let cg_len = side_len * (tile_len + 1);
        let mut char_grid: Vec<Vec<char>> = vec![vec![]; cg_len];

        println!("Grid: {:?}", grid);
        for (grid_idx, tile_idx) in grid.iter().enumerate() {
            let grid_row_offset = (grid_idx / side_len) * (tile_len + 1);
            if let Some(idx) = tile_idx {
                for (c_idx, c) in self.tiles[*idx].grid.iter().enumerate() {
                    let row = c_idx / tile_len;
                    char_grid[grid_row_offset + row].push(*c);
                    if (c_idx + 1) % tile_len == 0 {
                        char_grid[grid_row_offset + row].push(' ');
                    }
                }
            } else {
                for row in 0..tile_len {
                    for _ in 0..tile_len {
                        char_grid[grid_row_offset + row].push(' ');
                    }
                    char_grid[grid_row_offset + row].push(' ');
                }
            }
        }

        for row in char_grid {
            for c in row {
                print!("{}", c);
            }
            print!("\n");
        }
    }
}

#[derive(Debug, Clone)]
struct Picture {
    grid: Vec<char>,
    line_len: usize,
}

impl Picture {
    fn new(tiles: &[Tile], order: &[usize], tiles_per_side: usize) -> Self {
        let chars_per_tile = tiles[0].line_len - 2;
        let line_len = chars_per_tile * tiles_per_side;
        let mut grid = vec!['O'; line_len * line_len];
        for pic_row in 0..tiles_per_side {
            for pic_col in 0..tiles_per_side {
                let tile_idx = order[pic_row * tiles_per_side + pic_col];
                let tile = &tiles[tile_idx];
                let tile_offset = pic_row * chars_per_tile * line_len + pic_col * chars_per_tile;
                for tile_row in 1..tile.line_len - 1 {
                    for tile_col in 1..tile.line_len - 1 {
                        let pic_idx = tile_offset + (tile_row - 1) * line_len + (tile_col - 1);
                        let tile_grid_idx = tile_row * tile.line_len + tile_col;
                        grid[pic_idx] = tile.grid[tile_grid_idx];
                    }
                }
            }
        }
        Picture { grid, line_len }
    }

    fn monster_exists(&self) -> bool {
        let monster: Vec<char> = MONSTER.chars().filter(|c| *c != '\n').collect();
        let mon_width = MONSTER.lines().next().unwrap().len();
        let mon_height = MONSTER.lines().count();
        for row in 0..self.line_len - mon_height + 1 {
            for col in 0..self.line_len - mon_width + 1 {
                let mut matches = true;
                for mon_row in 0..mon_height {
                    for mon_col in 0..mon_width {
                        let mon_idx = mon_row * mon_width + mon_col;
                        if monster[mon_idx] == '#' {
                            let pic_idx = (row + mon_row) * self.line_len + col + mon_col;
                            if self.grid[pic_idx] != '#' {
                                matches = false;
                                break;
                            }
                        }
                    }
                    if !matches {
                        break;
                    }
                }
                if matches {
                    println!("Monster found");
                    return true;
                }
            }
        }
        false
    }

    fn mark_monsters(&mut self) {
        let monster: Vec<char> = MONSTER.chars().filter(|c| *c != '\n').collect();
        let mon_width = MONSTER.lines().next().unwrap().len();
        let mon_height = MONSTER.lines().count();
        for row in 0..self.line_len - mon_height + 1 {
            for col in 0..self.line_len - mon_width + 1 {
                let mut matches = true;
                for mon_row in 0..mon_height {
                    for mon_col in 0..mon_width {
                        let mon_idx = mon_row * mon_width + mon_col;
                        if monster[mon_idx] == '#' {
                            let pic_idx = (row + mon_row) * self.line_len + col + mon_col;
                            if self.grid[pic_idx] != '#' {
                                matches = false;
                                break;
                            }
                        }
                    }
                    if !matches {
                        break;
                    }
                }
                if matches {
                    // Mark the monster!
                    for mon_row in 0..mon_height {
                        for mon_col in 0..mon_width {
                            let mon_idx = mon_row * mon_width + mon_col;
                            if monster[mon_idx] == '#' {
                                let pic_idx = (row + mon_row) * self.line_len + col + mon_col;
                                self.grid[pic_idx] = 'O';
                            }
                        }
                        if !matches {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn count_roughness(&self) -> usize {
        self.grid.iter().filter(|c| **c == '#').count()
    }

    fn rotate_and_flip_until_monster_found(&mut self) {
        for idx in 0..5 {
            if self.monster_exists() {
                return;
            }
            println!("idx:{}\n{}", idx, self);
            self.rotate_right();
            if self.monster_exists() {
                return;
            }
            self.flip_horizontally();
            println!("idx:{}, flipped\n{}", idx, self);
            if self.monster_exists() {
                return;
            }
            self.flip_horizontally();
        }
    }

    fn rotate_right(&mut self) {
        let mut new_grid: Vec<char> = Vec::with_capacity(self.grid.len());
        for new_idx in 0..self.grid.len() {
            let new_row = new_idx / self.line_len;
            let new_col = new_idx % self.line_len;
            let old_row = (self.line_len - 1) - new_col;
            let old_col = new_row;
            let value = self.grid[old_row * self.line_len + old_col];
            new_grid.push(value);
        }
        self.grid = new_grid;
    }

    fn flip_horizontally(&mut self) {
        let mut new_grid: Vec<char> = Vec::with_capacity(self.grid.len());
        for new_idx in 0..self.grid.len() {
            let col = new_idx % self.line_len;
            let new_row = new_idx / self.line_len;
            let old_row = (self.line_len - 1) - new_row;
            let value = self.grid[old_row * self.line_len + col];
            new_grid.push(value);
        }
        self.grid = new_grid;
    }

    fn flip_vertically(&mut self) {
        let mut new_grid: Vec<char> = Vec::with_capacity(self.grid.len());
        for new_idx in 0..self.grid.len() {
            let row = new_idx / self.line_len;
            let new_col = new_idx % self.line_len;
            let old_col = (self.line_len - 1) - new_col;
            let value = self.grid[row * self.line_len + old_col];
            new_grid.push(value);
        }
        self.grid = new_grid;
    }
}

impl fmt::Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Printing picture!\n").unwrap();
        for y in 0..self.line_len {
            for x in 0..self.line_len {
                write!(f, "{}", self.grid[y * self.line_len + x]).unwrap();
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
    fn simple_star1() {
        let mut tiles = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&mut tiles), 1951 * 3079 * 2971 * 1171);
    }

    #[test]
    fn full_star1() {
        let mut tiles = parse_input(INPUT);
        assert_eq!(star1(&mut tiles), 108603771107737);
    }

    #[test]
    fn simple_star2() {
        let mut tiles = parse_input(SIMPLE_INPUT);
        assert_eq!(star2(&mut tiles), 273);
    }

    #[test]
    fn full_star2() {
        let mut tiles = parse_input(INPUT);
        assert_eq!(star2(&mut tiles), 2129);
    }
}

use std::collections::HashMap;
use std::fmt;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-20.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-20.txt");

fn main() {
    let tiles = parse_input(SIMPLE_INPUT);

    // Star 1
    star1(&tiles);
}

fn parse_input(input: &str) -> Vec<Tile> {
    input.trim().split("\n\n").map(Tile::new).collect()
}

fn star1(tiles: &[Tile]) -> u64 {
    let mut matcher = TileMatcher::new(tiles);
    matcher.find_matches();
    let tiles_per_matches = matcher.get_tiles_per_matches();

    println!("{:?}", tiles_per_matches);

    println!("Construct attempt");
    matcher.construct_picture();

    tiles_per_matches[2]
        .iter()
        .map(|idx| tiles[*idx].id as u64)
        .product()
}

fn star2(tiles: &[Tile]) -> usize {
    todo!();
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

    fn opposite_side(self) -> Self {
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
    len: usize,
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

        let top_border = lines.clone().next().unwrap().trim().chars().collect();
        let bottom_border = lines.clone().last().unwrap().trim().chars().collect();
        let left_border = lines
            .clone()
            .map(|line| line.trim().chars().next().unwrap())
            .collect();
        let right_border = lines
            .clone()
            .map(|line| line.trim().chars().last().unwrap())
            .collect();
        let borders: [Vec<char>; 4] = [top_border, right_border, bottom_border, left_border];
        let len = borders[0].len();

        let mut borders_flipped: [Vec<char>; 4] = borders.clone();
        for border in borders_flipped.iter_mut() {
            border.reverse();
        }

        let grid = lines.flat_map(|line| line.trim().chars()).collect();

        Tile {
            id,
            grid,
            borders,
            borders_flipped,
            len,
        }
    }

    fn find_match(&self, other: &Tile) -> Option<TileMatch> {
        let id1 = self.id;
        let id2 = other.id;
        for (idx1, border) in self.borders.iter().enumerate() {
            let found = other.borders.iter().position(|other| other == border);
            if let Some(idx2) = found {
                return Some(TileMatch::new(id1, id2, idx1, idx2, false));
            }

            // Now try with borders flipped
            let found = other
                .borders_flipped
                .iter()
                .position(|other| other == border);
            if let Some(idx2) = found {
                return Some(TileMatch::new(id1, id2, idx1, idx2, true));
            }
        }
        None
    }

    fn rotate_right(&mut self) {
        let mut new_grid: Vec<char> = Vec::with_capacity(self.grid.len());
        for new_idx in 0..self.grid.len() {
            let new_row = new_idx / self.len;
            let new_col = new_idx % self.len;
            let old_row = (self.len - 1) - new_col;
            let old_col = new_row;
            let value = self.grid[old_row * self.len + old_col];
            new_grid.push(value);
        }
        self.grid = new_grid;
        // TODO: Update borders and flipped borders
    }

    fn flip_horizontally(&mut self) {
        let mut new_grid: Vec<char> = Vec::with_capacity(self.grid.len());
        for new_idx in 0..self.grid.len() {
            let col = new_idx % self.len;
            let new_row = new_idx / self.len;
            let old_row = (self.len - 1) - new_row;
            let value = self.grid[old_row * self.len + col];
            new_grid.push(value);
        }
        self.grid = new_grid;
        // TODO: Update borders and flipped borders
    }

    fn flip_vertically(&mut self) {
        let mut new_grid: Vec<char> = Vec::with_capacity(self.grid.len());
        for new_idx in 0..self.grid.len() {
            let row = new_idx / self.len;
            let new_col = new_idx % self.len;
            let old_col = (self.len - 1) - new_col;
            let value = self.grid[row * self.len + old_col];
            new_grid.push(value);
        }
        self.grid = new_grid;
        // TODO: Update borders and flipped borders
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.len {
            for x in 0..self.len {
                write!(f, "{}", self.grid[y * self.len + x]).unwrap();
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
    border2: Side,
    flipped: bool,
}

impl TileMatch {
    fn new(id1: u32, id2: u32, idx1: usize, idx2: usize, flipped: bool) -> TileMatch {
        TileMatch {
            id1: id1,
            id2: id2,
            border1: Side::from(idx1 as u8),
            border2: Side::from(idx2 as u8),
            flipped,
        }
    }
}

struct TileMatcher<'a> {
    tiles: &'a [Tile],
    tile_match: Vec<Vec<TileMatch>>,
    id_idx_map: HashMap<u32, usize>,
}

impl<'a> TileMatcher<'a> {
    fn new(tiles: &[Tile]) -> TileMatcher {
        let id_idx_map = tiles
            .iter()
            .enumerate()
            .map(|(idx, tile)| (tile.id, idx))
            .collect();
        TileMatcher {
            tiles,
            tile_match: vec![Vec::new(); tiles.len()],
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
        let mut grid: Vec<Option<(usize, bool, bool, u8)>> = Vec::new();
        grid.resize(side_len * side_len, None);

        // Top left tile
        let mut tile_idx = tiles_per_matches[2][0];
        let mut inserted_tile_side: Side;
        let mut inserted_tile_match: TileMatch;
        {
            let matches = &self.tile_match[tile_idx];
            let mut times_rotated = 0;
            let mut side1 = matches[0].border1;
            let mut side2 = matches[1].border1;
            while !((side1 == Side::Right && side2 == Side::Bottom)
                || (side1 == Side::Bottom && side2 == Side::Right))
            {
                self.rotate_tile_right(tile_idx);
                let matches = &self.tile_match[tile_idx];
                side1 = matches[0].border1;
                side2 = matches[1].border1;
                times_rotated += 1;
            }
            grid[0] = Some((tile_idx, false, false, times_rotated));

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
        {
            for col_idx in 1..side_len {
                // First, find the match to the previously inserted tile
                let tm_idx = self.find_tile_match_idx(tile_idx, inserted_tile_match.id1);

                // Then, rotate the tile until it fits correctly
                let times_rotated = self.rotate_tile_until_tile_match_opposite(
                    tile_idx,
                    tm_idx,
                    inserted_tile_side,
                );

                // Check if it needed to be flipped along the horizon to match
                let flipped_h = inserted_tile_match.flipped;
                if flipped_h {
                    self.flip_tile(tile_idx, inserted_tile_side);
                }

                // Then, insert it into the grid
                grid[col_idx] = Some((tile_idx, flipped_h, false, times_rotated));

                // Throw current tile out
                self.tile_match[tile_idx].remove(tm_idx);

                // Rotate if needed
                if col_idx == side_len - 1 {
                    inserted_tile_side = Side::Bottom;
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

        // Right column
        {
            for row_idx in 1..side_len {
                // First, find the match to the previously inserted tile
                let tm_idx = self.find_tile_match_idx(tile_idx, inserted_tile_match.id1);

                // Then, rotate the tile until it fits correctly
                let times_rotated = self.rotate_tile_until_tile_match_opposite(
                    tile_idx,
                    tm_idx,
                    inserted_tile_side,
                );

                // Check if it needed to be flipped along the horizon to match
                let flipped_v = inserted_tile_match.flipped;
                if flipped_v {
                    self.flip_tile(tile_idx, inserted_tile_side);
                }

                // Then, insert it into the grid
                grid[row_idx * side_len + (side_len - 1)] =
                    Some((tile_idx, false, flipped_v, times_rotated));

                // Throw current tile out
                self.tile_match[tile_idx].remove(tm_idx);

                // Rotate if needed
                if row_idx == side_len - 1 {
                    inserted_tile_side = Side::Left;
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

        // Bottom row
        {
            for col_idx in (0..side_len - 1).rev() {
                // First, find the match to the previously inserted tile
                let tm_idx = self.find_tile_match_idx(tile_idx, inserted_tile_match.id1);

                // Then, rotate the tile until it fits correctly
                let times_rotated = self.rotate_tile_until_tile_match_opposite(
                    tile_idx,
                    tm_idx,
                    inserted_tile_side,
                );

                // Check if it needed to be flipped along the horizon to match
                let flipped_h = inserted_tile_match.flipped;
                if flipped_h {
                    self.flip_tile(tile_idx, inserted_tile_side);
                }

                // Then, insert it into the grid
                grid[(side_len - 1) * side_len + col_idx] =
                    Some((tile_idx, flipped_h, false, times_rotated));

                // Throw current tile out
                self.tile_match[tile_idx].remove(tm_idx);

                // Rotate if needed
                if col_idx == 0 {
                    inserted_tile_side = Side::Top;
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

        // Left column
        {
            for row_idx in (1..side_len).rev() {
                // First, find the match to the previously inserted tile
                let tm_idx = self.find_tile_match_idx(tile_idx, inserted_tile_match.id1);

                // Then, rotate the tile until it fits correctly
                let times_rotated = self.rotate_tile_until_tile_match_opposite(
                    tile_idx,
                    tm_idx,
                    inserted_tile_side,
                );

                // Check if it needed to be flipped along the horizon to match
                let flipped_v = inserted_tile_match.flipped;
                if flipped_v {
                    self.flip_tile(tile_idx, inserted_tile_side);
                }

                // Then, insert it into the grid
                grid[row_idx * side_len] = Some((tile_idx, false, flipped_v, times_rotated));

                // Throw current tile out
                self.tile_match[tile_idx].remove(tm_idx);

                /*
                // Rotate if needed
                if row_idx == 1 {
                    inserted_tile_side = Side::Right;
                }
                // Now, find the next tile
                let tm_idx = self.tile_match[tile_idx]
                    .iter()
                    .position(|tm| tm.border1 == inserted_tile_side)
                    .unwrap();
                inserted_tile_match = self.tile_match[tile_idx].remove(tm_idx);
                tile_idx = self.id_idx_map[&inserted_tile_match.id2];
                */
            }
        }

        // ! Only works for one row atm!!!
        let cg_len = (side_len - 1) * self.tiles[0].len;
        let mut char_grid: Vec<Vec<char>> = vec![vec![]; cg_len];

        for idx in 0..side_len {
            let (idx, flipped_h, flipped_v, rotated) = grid[idx].unwrap();
            let mut tile = self.tiles[idx].clone();
            if flipped_h {
                tile.flip_horizontally();
            }
            if flipped_v {
                tile.flip_vertically();
            }
            for _ in 0..rotated {
                tile.rotate_right();
            }
            for (c_idx, c) in tile.grid.iter().enumerate() {
                let row = c_idx / tile.len;
                char_grid[row].push(*c);
            }

            for row in 0..tile.len {
                char_grid[row].push(' ');
            }
        }

        for row in char_grid {
            for c in row {
                print!("{}", c);
            }
            print!("\n");
        }

        Picture {}
    }

    fn rotate_tile_right(&mut self, tile_idx: usize) {
        for tile_match_idx in 0..self.tile_match[tile_idx].len() {
            let mut tile_match = &mut self.tile_match[tile_idx][tile_match_idx];
            let rotated_border = Side::from(tile_match.border1).rotate_right().into();
            tile_match.border1 = rotated_border;

            // Fix other match (if it exists)
            let tile_id = tile_match.id1;
            let other_match_tile_idx = self.id_idx_map[&tile_match.id2];
            self.tile_match[other_match_tile_idx]
                .iter_mut()
                .find(|other_match| other_match.id2 == tile_id)
                .map(|tm| tm.border2 = rotated_border);
        }
        // We may need to flip!
        // left -> top, right -> bottom
        for tile_match_idx in 0..self.tile_match[tile_idx].len() {
            let tile_match = self.tile_match[tile_idx][tile_match_idx];
            if tile_match.border1 == Side::Top || tile_match.border1 == Side::Bottom {
                self.flip_tile(tile_idx, tile_match.border1);
            }
        }
    }

    fn rotate_tile_until_tile_match_opposite(
        &mut self,
        tile_idx: usize,
        tm_idx: usize,
        opposite_side: Side,
    ) -> u8 {
        let wanted_side = opposite_side.opposite_side();
        let mut times_rotated = 0;
        while self.tile_match[tile_idx][tm_idx].border1 != wanted_side {
            self.rotate_tile_right(tile_idx);
            times_rotated += 1;
        }
        times_rotated
    }

    fn flip_tile(&mut self, tile_idx: usize, inserted_side: Side) {
        let opposite_side = inserted_side.opposite_side();

        // Flip my entries
        let mut flipped_entries = Vec::new();
        for tm_idx in 0..self.tile_match[tile_idx].len() {
            let tm = &mut self.tile_match[tile_idx][tm_idx];
            if tm.border1 == inserted_side || tm.border1 == opposite_side {
                tm.flipped = !tm.flipped;
                flipped_entries.push(tm_idx);
            }
        }

        // Flip the other matches in case they exist
        for tm_idx in flipped_entries.iter() {
            let tm = &self.tile_match[tile_idx][*tm_idx];
            let current_id = tm.id1;
            let other_idx = self.id_idx_map[&tm.id2];
            self.tile_match[other_idx]
                .iter_mut()
                .find(|tm| tm.id2 == current_id)
                .map(|tm| tm.flipped = !tm.flipped);
        }
    }

    fn find_tile_match_idx(&self, tile_idx: usize, other_id: u32) -> usize {
        self.tile_match[tile_idx]
            .iter()
            .position(|tm| tm.id2 == other_id)
            .unwrap()
    }
}

struct Picture {
    // tiles: Vec<Tile>,
// side_len: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let tiles = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&tiles), 1951 * 3079 * 2971 * 1171);
    }

    #[test]
    fn full_star1() {
        let tiles = parse_input(INPUT);
        assert_eq!(star1(&tiles), 108603771107737);
    }

    #[test]
    fn simple_star2() {
        let tiles = parse_input(SIMPLE_INPUT);
        assert_eq!(star2(&tiles), 273);
    }

    #[test]
    fn full_star2() {
        let tiles = parse_input(INPUT);
        assert_eq!(star2(&tiles), 259172170858496);
    }
}

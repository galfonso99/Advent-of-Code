use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}
use Tile::*;
impl Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => NorthSouth,
            '-' => EastWest,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            'S' => Start,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    row_idx: usize,
    col_idx: usize,
}

impl Coord {
    fn new(row_idx: usize, col_idx: usize) -> Self {
        Self { row_idx, col_idx }
    }

    fn valid_neighbours(&self, map: &[Vec<Tile>]) -> Vec<Coord> {
        let mut neighbours = vec![];
        let max_height = map.len() - 1;
        let max_width = map[0].len() - 1;

        match map[self.row_idx][self.col_idx] {
            Ground => (),
            Start => {
                // north
                if self.row_idx > 0 {
                    let tile = map[self.row_idx - 1][self.col_idx];
                    if matches!(tile, NorthSouth | SouthWest | SouthEast) {
                        neighbours.push(Coord::new(self.row_idx - 1, self.col_idx));
                    }
                }
                // south
                if self.row_idx < max_height {
                    let tile = map[self.row_idx + 1][self.col_idx];
                    if matches!(tile, NorthSouth | NorthWest | NorthEast) {
                        neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                    }
                }
                // west
                if self.col_idx > 0 {
                    let tile = map[self.row_idx][self.col_idx - 1];
                    if matches!(tile, EastWest | SouthEast | NorthEast) {
                        neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                    }
                }
                // east
                if self.col_idx < max_width {
                    let tile = map[self.row_idx][self.col_idx + 1];
                    if matches!(tile, EastWest | NorthWest | SouthWest) {
                        neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                    }
                }
            }
            NorthSouth => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // south
                if self.row_idx < max_height && map[self.row_idx + 1][self.col_idx] != Ground {
                    match map[self.row_idx + 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
            }
            EastWest => {
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
            NorthEast => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
            NorthWest => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
            }
            SouthWest => {
                // south
                if self.row_idx < max_height {
                    match map[self.row_idx + 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
            }
            SouthEast => {
                // south
                if self.row_idx < max_height {
                    match map[self.row_idx + 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
        }

        neighbours
    }
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Coord) {
    let mut start = Coord::new(0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    let tile = Tile::from(c);
                    if tile == Start {
                        start = Coord::new(row_idx, col_idx)
                    }
                    tile
                })
                .collect()
        })
        .collect();
    (map, start)
}

fn build_loop(start: Coord, map: &[Vec<Tile>]) -> HashSet<Coord> {
    let mut loop_coords = HashSet::new();
    loop_coords.insert(start);
    let mut to_visit = start.valid_neighbours(map);

    while let Some(curr_pos) = to_visit.pop() {
        for neighbour in curr_pos.valid_neighbours(map) {
            if !loop_coords.contains(&neighbour) {
                to_visit.push(neighbour);
                loop_coords.insert(neighbour);
            }
        }
    }

    loop_coords
}

fn get_start_pipe(map: &Vec<Vec<Tile>>, start: Coord) -> Tile {
    let neighbours = start.valid_neighbours(map);
    let north = neighbours
        .iter()
        .find(|coord| coord.row_idx < start.row_idx)
        .is_some();
    let south = neighbours
        .iter()
        .find(|coord| coord.row_idx > start.row_idx)
        .is_some();
    let west = neighbours
        .iter()
        .find(|coord| coord.col_idx < start.col_idx)
        .is_some();
    let east = neighbours
        .iter()
        .find(|coord| coord.col_idx > start.col_idx)
        .is_some();

    match (north, west, south, east) {
        (true, true, _, _) => NorthWest,
        (true, _, true, _) => NorthSouth,
        (true, _, _, true) => NorthEast,
        (_, true, true, _) => SouthWest,
        (_, _, true, true) => SouthEast,
        (_, true, _, true) => EastWest,
        _ => panic!("No valid tile to replace Start with was found"),
    }
}

/// replace start with a valid pipe segment, and only keep pipe segments that are part of the loop
fn clean_map(start: Coord, loop_coords: &HashSet<Coord>, map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let start_pipe = get_start_pipe(&map, start);

    map.into_iter()
        .enumerate()
        .map(|(row_idx, line)| {
            line.into_iter()
                .enumerate()
                .map(|(col_idx, tile)| match tile {
                    Start => start_pipe,
                    pipe if loop_coords.contains(&Coord::new(row_idx, col_idx)) => pipe,
                    _ => Ground,
                })
                .collect()
        })
        .collect()
}

pub fn part_2(input: &str) -> usize {
    let (map, start) = parse(input);
    let loop_coords = build_loop(start, &map);
    let map = clean_map(start, &loop_coords, map);
    let mut inside = false;
    map.into_iter()
        .flatten()
        .filter(|tile| match tile {
            Ground => inside,
            NorthSouth | NorthWest | NorthEast => {
                inside = !inside;
                false
            }
            _ => false,
        })
        .count()
}

pub fn main() {
    let input = include_str!("input");
    let res = part_2(input);
    println!("{}", res);
}

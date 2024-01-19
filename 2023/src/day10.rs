#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Path {
    char: char,
    y: usize,
    x: usize,
    dir: i8,
    step: usize,
}
pub fn main() {
    // part1();
    part2();
    // Standard chars
    // | - F L 7 J
    // Box drawing chars
    // ━ ┃ ┏ ┗ ┓ ┛
}

fn part1() {
    let file = File::open("test").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input).unwrap();
    let mut chars = input
        .lines()
        .map(|str| str.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut paths: Vec<Path> = Vec::new();
    // Find animal position 'S'
    let pos = chars
        .iter()
        .enumerate()
        .map(|(i, arr)| {
            arr.iter()
                .enumerate()
                .map(move |(j, c)| if c == &'S' { Some((i, j)) } else { None })
                .find(|x| x.is_some())
        })
        .flatten()
        .next()
        .unwrap()
        .unwrap();
    find_initial_paths(&mut paths, &pos, &mut chars);
    // println!("{:?}", paths);
    // println!("{:?}", chars);
    while paths[0].x != paths[1].x || paths[0].y != paths[1].y {
        for i in 0..2 {
            get_next_step(&mut paths[i], &mut chars);
        }
    }
    // println!("{:?}", paths);
    // println!("{:?}", chars);
    chars
        .iter()
        .for_each(|str| println!("{}", str.iter().collect::<String>()));
    // println!("{}", paths[0].step);
}

fn get_next_step(path: &mut Path, chars: &mut Vec<Vec<char>>) {
    match path.dir {
        1 => {
            let dir = get_next_dir(chars[path.y - 1][path.x], path.dir);
            *path = Path {
                char: chars[path.y - 1][path.x],
                y: path.y - 1,
                x: path.x,
                dir,
                step: path.step + 1,
            };
        }
        -1 => {
            let dir = get_next_dir(chars[path.y + 1][path.x], path.dir);
            *path = Path {
                char: chars[path.y + 1][path.x],
                y: path.y + 1,
                x: path.x,
                dir,
                step: path.step + 1,
            };
        }
        2 => {
            let dir = get_next_dir(chars[path.y][path.x + 1], path.dir);
            *path = Path {
                char: chars[path.y][path.x + 1],
                y: path.y,
                x: path.x + 1,
                dir,
                step: path.step + 1,
            };
        }
        -2 => {
            let dir = get_next_dir(chars[path.y][path.x - 1], path.dir);
            *path = Path {
                char: chars[path.y][path.x - 1],
                y: path.y,
                x: path.x - 1,
                dir,
                step: path.step + 1,
            };
        }
        _ => {
            *path = Path {
                char: chars[path.y - 1][path.x],
                y: path.y - 1,
                x: path.x,
                dir: 1,
                step: path.step + 1,
            };
        }
    };
}

fn get_next_dir(char: char, dir: i8) -> i8 {
    match char {
        '|' => dir,
        '-' => dir,
        'F' if dir == -2 => -1,
        'F' => 2,
        'L' if dir == -2 => 1,
        'L' => 2,
        '7' if dir == 2 => -1,
        '7' => -2,
        'J' if dir == 2 => 1,
        'J' => -2,
        _ => dir,
    }
}

fn find_initial_paths(paths: &mut Vec<Path>, pos: &(usize, usize), chars: &mut Vec<Vec<char>>) {
    if paths.len() < 2 && pos.0 > 0 {
        if chars[pos.0 - 1][pos.1] == '|'
            || chars[pos.0 - 1][pos.1] == 'F'
            || chars[pos.0 - 1][pos.1] == '7'
        {
            let dir = get_next_dir(chars[pos.0 - 1][pos.1], 1);
            paths.push(Path {
                char: chars[pos.0 - 1][pos.1],
                y: pos.0 - 1,
                x: pos.1,
                dir,
                step: 1,
            });
            chars[pos.0 - 1][pos.1] = map(chars[pos.0 - 1][pos.1]);
        }
    }
    if paths.len() < 2 && pos.1 + 1 < chars[pos.0].len() {
        if chars[pos.0][pos.1 + 1] == '-'
            || chars[pos.0][pos.1 + 1] == 'J'
            || chars[pos.0][pos.1 + 1] == '7'
        {
            let dir = get_next_dir(chars[pos.0][pos.1 + 1], 2);
            paths.push(Path {
                char: chars[pos.0][pos.1 + 1],
                y: pos.0,
                x: pos.1 + 1,
                dir,
                step: 1,
            });
            chars[pos.0][pos.1 + 1] = map(chars[pos.0][pos.1 + 1]);
        }
    }
    if paths.len() < 2 && pos.0 + 1 < chars.len() {
        if chars[pos.0 + 1][pos.1] == '|'
            || chars[pos.0 + 1][pos.1] == 'J'
            || chars[pos.0 + 1][pos.1] == 'L'
        {
            let dir = get_next_dir(chars[pos.0 + 1][pos.1], -1);
            paths.push(Path {
                char: chars[pos.0 + 1][pos.1],
                y: pos.0 + 1,
                x: pos.1,
                dir,
                step: 1,
            });
            chars[pos.0 + 1][pos.1] = map(chars[pos.0 + 1][pos.1]);
        }
    }
    if paths.len() < 2 && pos.1 > 0 {
        if chars[pos.0][pos.1 - 1] == '-'
            || chars[pos.0][pos.1 - 1] == 'F'
            || chars[pos.0][pos.1 - 1] == 'L'
        {
            let dir = get_next_dir(chars[pos.0][pos.1 - 1], -2);
            paths.push(Path {
                char: chars[pos.0][pos.1 - 1],
                y: pos.0,
                x: pos.1 - 1,
                dir,
                step: 1,
            });
            chars[pos.0][pos.1 - 1] = map(chars[pos.0][pos.1 - 1]);
        }
    }
}

// fn part2() {
//     // With my current idea there is no way to know if a tile is outside of the loop via an
//     // unclosed loop like 7L which are not connected
//     let file = File::open("input").unwrap();
//     let mut buf_reader = BufReader::new(file);
//     let mut input = String::new();
//     buf_reader.read_to_string(&mut input).unwrap();
//     // let pretty = input
//     //     .replace('-', "━")
//     //     .replace('|', "┃")
//     //     .replace('F', "┏")
//     //     .replace('L', "┗")
//     //     .replace('7', "┓")
//     //     .replace('J', "┛");
//     // println!("{}", pretty);
//     // println!("{}", input);
//     let mut chars = input
//         .lines()
//         .map(|str| str.chars().collect::<Vec<char>>())
//         .collect::<Vec<Vec<char>>>();
//     let mut paths: Vec<Path> = Vec::new();
//     // Find animal position 'S'
//     let pos = chars.iter().enumerate()
//         .map(|(i, arr)| {
//             arr.iter()
//                 .enumerate()
//                 .map(move |(j, c)| if c == &'S' { Some((i, j)) } else { None })
//                 .find(|x| x.is_some())
//         })
//         .flatten().next().unwrap().unwrap();
//     find_initial_paths(&mut paths, &pos, &mut chars);
//     // let initial_dirs = vec![(paths[0].x, paths[0].y), (paths[1].x, paths[1].y)];
//     let initial_dirs = (paths[0].dir, paths[1].dir);
//     let first_inside_tile = find_first_tile(&pos, initial_dirs);
//     // println!("{:?}", paths);
//     // println!("{:?}", chars);
//     while paths[0].x != paths[1].x || paths[0].y != paths[1].y {
//         for i in 0..2 {
//             get_next_step_2(&mut paths[i], &mut chars);
//         }
//     }
//     for i in 0..chars.len() {
//         let len = chars[i].len();
//         if i == 0 || i == chars.len() - 1 {
//             for j in 0..chars[i].len() {
//                 chars[i][j] = 'E';
//             }
//         } else {
//             chars[i][0] = 'E';
//             chars[i][len - 1] = 'E';
//         }
//     }
//     // chars
//     //     .iter()
//     //     .for_each(|str| println!("{}", str.iter().collect::<String>()));
//     for i in 0..chars.len() {
//         let len = chars[i].len();
//         if i == 0 {
//             for j in 1..chars[i].len() - 1 {
//                 explore_maze(chars[i+1][j], i+1, j, &mut chars);
//             }
//         } else if i == chars.len() - 1 {
//             for j in 1..chars[i].len() - 1 {
//                 explore_maze(chars[i-1][j], i-1, j, &mut chars);
//             }
//         } else {
//             explore_maze(chars[i][1], i, 1, &mut chars);
//             explore_maze(chars[i][len-2], i, len-2, &mut chars);
//         }
//     }
//     // println!("{:?}", paths);
//     // println!("{:?}", chars);
//     // chars = chars.iter().map(|arr| arr.iter().map(|&c| if c != '%' {'^'} else {c}).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
//     chars
//         .iter()
//         .for_each(|str| println!("{}", str.iter().collect::<String>()));
//     // println!("{}", paths[0].step);
// }

// fn explore_maze(char: char, y: usize, x: usize, dir: [i8; 2], chars: &mut Vec<Vec<char>>) {
//     if char == 'E' {return;}
//     if is_blocked(dir, char) {return;}
//     // if char == '━' || char == '┃' || char == '┏' || char == '┗' || char == '┓' || char == '┛' {return;}
//     if char == '.' || char == '-' || char == '|' || char == 'F' || char == 'L' || char == '7' || char == 'J' {
//         chars[y][x] = 'E';
//     }
//     let dirs: [[i8; 2]; 8] = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];
//     for i in 0..8 {
//         let delta_y = (y as i32 + dirs[i][0] as i32) as usize;
//         let delta_x = (x as i32 + dirs[i][1] as i32) as usize;
//         // println!("{}: {}: {}: {}: {}: {}", char, y, x, delta_y, delta_x, chars[delta_y][delta_x]);
//         // println!("{}: {}: {}: {}: {}", char, y, x, delta_y, delta_x);
//         explore_maze(chars[delta_y][delta_x], delta_y, delta_x, dirs[i], chars);
//     }
// }

fn get_next_step_2(path: &mut Path, chars: &mut Vec<Vec<char>>) {
    match path.dir {
        1 => {
            let dir = get_next_dir(chars[path.y - 1][path.x], path.dir);
            *path = Path { char: chars[path.y - 1][path.x], y: path.y - 1, x: path.x, dir, step: path.step + 1, };
            chars[path.y][path.x] = map(chars[path.y][path.x]);
        }
        -1 => {
            let dir = get_next_dir(chars[path.y + 1][path.x], path.dir);
            *path = Path { char: chars[path.y + 1][path.x], y: path.y + 1, x: path.x, dir, step: path.step + 1 };
            chars[path.y][path.x] = map(chars[path.y][path.x]);
        }
        2 => {
            let dir = get_next_dir(chars[path.y][path.x + 1], path.dir);
            *path = Path { char: chars[path.y][path.x + 1], y: path.y, x: path.x + 1, dir, step: path.step + 1, };
            chars[path.y][path.x] = map(chars[path.y][path.x]);
        }
        -2 => {
            let dir = get_next_dir(chars[path.y][path.x - 1], path.dir);
            *path = Path { char: chars[path.y][path.x - 1], y: path.y, x: path.x - 1, dir, step: path.step + 1, };
            chars[path.y][path.x] = map(chars[path.y][path.x]);
        }
        _ => {
            *path = Path { char: chars[path.y - 1][path.x], y: path.y - 1, x: path.x, dir: 1, step: path.step + 1, };
            chars[path.y][path.x] = map(chars[path.y][path.x]);
        }
    };
}

// fn is_blocked(dir: [i8; 2], char: char) -> bool {
//     match char {
//         '━' => (dir[0] == -1 || dir[0] == 1) && dir[1] == 0,
//         '┃' => (dir[1] == -1 || dir[1] == 1) && dir[0] == 0,
//         '┏' => {},
//         '┗' => {},
//         '┓' => {},
//         '┛' => {},
//     }
// }

fn find_first_tile(pos: &(usize, usize), initial_dirs: (i8, i8)) -> (usize, usize) {
    let (dir_x, dir_y) = if initial_dirs.0 < 0 {initial_dirs} else {(initial_dirs.1, initial_dirs.0)};
    let dir_x: isize = if dir_x < -1 {-1} else {1};
    let dir_y: isize = if dir_y > 1 {-1} else {1};
    let first_tile = ((pos.0 as isize + dir_y) as usize, (pos.1 as isize + dir_x) as usize);
    first_tile
}
fn map(char: char) -> char {
    match char {
        '|' => '┃',
        '-' => '━',
        'F' => '┏',
        'L' => '┗',
        '7' => '┓',
        'J' => '┛',
        _ => char
    }
}


use std::ops::Index;

// pub fn part1(input: &str) -> u16 {
//     let grid = Grid::from_str(input).unwrap();
//     let (start_row, start_col) = grid.find(b'S').unwrap();
//     let (mut last_row, mut last_col) = grid.get_s_neighbors((start_row, start_col)).next().unwrap();
//     let (mut curr_row, mut curr_col) = (start_row, start_col);
//
//     let mut lenght = 0;
//     while !(curr_row == start_row && curr_col == start_col) || lenght == 0 {
//         let (next_row, next_col) = grid.get_neighbor((curr_row, curr_col), (last_row, last_col));
//         (last_row, last_col) = (curr_row, curr_col);
//         (curr_row, curr_col) = (next_row, next_col);
//         lenght += 1;
//     }
//     lenght / 2
// }

pub fn part2() {
    let file = File::open("input").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input).unwrap();
    let grid = Grid::from_str(&input).unwrap();
    let (start_row, start_col) = grid.find(b'S').unwrap();
    let (mut last_row, mut last_col) = grid.get_s_neighbors((start_row, start_col)).next().unwrap();
    let (mut curr_row, mut curr_col) = (start_row, start_col);

    let mut lenght = 0;
    let mut twice_area = 0;
    while !(curr_row == start_row && curr_col == start_col) || lenght == 0 {
        let (next_row, next_col) = grid.get_neighbor((curr_row, curr_col), (last_row, last_col));
        (last_row, last_col) = (curr_row, curr_col);
        (curr_row, curr_col) = (next_row, next_col);
        lenght += 1;
        twice_area += (last_col + curr_col) * (last_row - curr_row);
    }
    let res = (twice_area.abs() - lenght) / 2 + 1;
    println!("{}", res);
}

struct Grid<'a> {
    inner: &'a [u8],
    modulo: i16,
    col_len: i16,
}

impl<'a> Grid<'a> {
    fn from_str(input: &'a str) -> Option<Self> {
        Some(Self {
            inner: input.as_bytes(),
            modulo: input.find('\n')? as i16 + 1,
            col_len: input.lines().count() as i16,
        })
    }

    fn find(&self, haystack: u8) -> Option<(i16, i16)> {
        let idx = self.inner.iter().position(|&pipe| pipe == haystack)? as i16;
        let row = idx / self.modulo;
        let col = idx % self.modulo;
        Some((row, col))
    }

    fn get_s_neighbors(
        &self,
        (curr_row, curr_col): (i16, i16),
    ) -> impl Iterator<Item = (i16, i16)> + '_ {
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .into_iter()
            .filter(move |&(d_row, d_col)| {
                curr_row + d_row >= 0
                    && curr_row + d_row < self.col_len
                    && curr_col + d_col >= 0
                    && curr_col + d_col < self.modulo - 1
            })
            .map(move |(d_row, d_col)| ((d_row, d_col), self[(curr_row + d_row, curr_col + d_col)]))
            .filter(|neighbor| {
                matches!(
                    neighbor,
                    ((-1, 0), b'|' | b'7' | b'F')
                        | ((0, -1), b'-' | b'L' | b'F')
                        | ((1, 0), b'|' | b'L' | b'J')
                        | ((0, 1), b'-' | b'J' | b'7')
                )
            })
            .map(move |((d_row, d_col), _)| (curr_row + d_row, curr_col + d_col))
    }

    fn get_neighbor(
        &self,
        (curr_row, curr_col): (i16, i16),
        (last_row, last_col): (i16, i16),
    ) -> (i16, i16) {
        let last_d_row = last_row - curr_row;
        let last_d_col = last_col - curr_col;
        match self[(curr_row, curr_col)] {
            b'|' | b'-' => (curr_row - last_d_row, curr_col - last_d_col),
            b'L' | b'7' => (curr_row - last_d_col, curr_col - last_d_row),
            b'J' | b'F' => (curr_row + last_d_col, curr_col + last_d_row),
            // (curr_row, curr_col) -> S
            _ => self
                .get_s_neighbors((curr_row, curr_col))
                .find(|&pipe| pipe != (last_row, last_col))
                .unwrap(),
        }
    }
}

impl Index<(i16, i16)> for Grid<'_> {
    type Output = u8;

    fn index(&self, (row, col): (i16, i16)) -> &Self::Output {
        &self.inner[(row * self.modulo + col) as usize]
    }
}

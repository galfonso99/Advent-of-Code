use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::os::windows;

#[derive(Debug, Clone)]
struct Path {
    char: char,
    y: usize,
    x: usize,
    dir: i8,
    step: usize,
}
pub fn main() {
    part2();
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

fn part2() {
    let file = File::open("test").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input).unwrap();
    let mut chars = input
        .lines()
        .map(|str| str.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    // Find animal position 'S'
    let pos = chars.iter().enumerate()
        .map(|(i, arr)| {
            arr.iter()
                .enumerate()
                .map(move |(j, c)| if c == &'S' { Some((i, j)) } else { None })
                .find(|x| x.is_some())
        })
        .flatten().next().unwrap().unwrap();

    let first_tile = Path {char: 'S', y: pos.0, x: pos.1, dir: 3, step: 0};
    let mut paths: Vec<Path> = vec![first_tile];
    find_initial_paths(&mut paths, &pos, &mut chars);

    let mut next_path = paths[1].clone();
    while next_path.char != 'S' {
        next_path = get_next_step_2(&next_path, &mut chars, &mut paths);
    }
    let len = paths.len();
    let twice_area = get_area(&mut paths);
    let inside_points = get_inside_points(twice_area, paths.len() - 1);
    println!("{}", inside_points);
}

fn get_area(paths: &mut Vec<Path>) -> isize {
    let sum: isize = paths.windows(2).map(|sl| (sl[1].y as isize * sl[0].x as isize) - (sl[1].x * sl[0].y) as isize).sum(); 
    let area = sum.abs();
    area
}
fn get_inside_points(twice_area: isize, len: usize) -> isize {
    let inside = (twice_area - len as isize) / 2 + 1;
    inside
}
fn get_next_step_2(path: &Path, chars: &mut Vec<Vec<char>>, paths: &mut Vec<Path>) -> Path {
    match path.dir {
        1 => {
            let dir = get_next_dir(chars[path.y - 1][path.x], path.dir);
            paths.push( Path { char: chars[path.y - 1][path.x], y: path.y - 1, x: path.x, dir, step: path.step + 1, });
            // chars[path.y][path.x] = map(chars[path.y][path.x]);
            Path { char: chars[path.y - 1][path.x], y: path.y - 1, x: path.x, dir, step: path.step + 1, }
        }
        -1 => {
            let dir = get_next_dir(chars[path.y + 1][path.x], path.dir);
            paths.push(Path { char: chars[path.y + 1][path.x], y: path.y + 1, x: path.x, dir, step: path.step + 1 });
            // chars[path.y][path.x] = map(chars[path.y][path.x]);
            Path { char: chars[path.y + 1][path.x], y: path.y + 1, x: path.x, dir, step: path.step + 1 }
        }
        2 => {
            let dir = get_next_dir(chars[path.y][path.x + 1], path.dir);
            paths.push(Path { char: chars[path.y][path.x + 1], y: path.y, x: path.x + 1, dir, step: path.step + 1, });
            // chars[path.y][path.x] = map(chars[path.y][path.x]);
            Path { char: chars[path.y][path.x + 1], y: path.y, x: path.x + 1, dir, step: path.step + 1, }
        }
        -2 => {
            let dir = get_next_dir(chars[path.y][path.x - 1], path.dir);
            paths.push(Path { char: chars[path.y][path.x - 1], y: path.y, x: path.x - 1, dir, step: path.step + 1, });
            // chars[path.y][path.x] = map(chars[path.y][path.x]);
            Path { char: chars[path.y][path.x - 1], y: path.y, x: path.x - 1, dir, step: path.step + 1, }
        }
        _ => {
            paths.push(Path { char: chars[path.y - 1][path.x], y: path.y - 1, x: path.x, dir: 1, step: path.step + 1, });
            // chars[path.y][path.x] = map(chars[path.y][path.x]);
            Path { char: chars[path.y - 1][path.x], y: path.y - 1, x: path.x, dir: 1, step: path.step + 1, }
        }
    }
}

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


#[derive(Debug, Clone)]
struct Tile {
    char: char,
    y: usize,
    x: usize,
    dir: Dir,
}
#[derive(Debug, Clone, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
pub fn main() {
    part1();
    part2();
}
fn part1() {
    let input = include_str!("input");
    let mut chars = input
        .lines()
        .map(|str| str.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let pos = chars.iter().enumerate()
        .map(|(i, arr)| {
            arr.iter()
                .enumerate()
                .map(move |(j, c)| if c == &'S' { Some((i, j)) } else { None })
                .find(|x| x.is_some())
        })
        .flatten().next().unwrap().unwrap();
    let first_tile = Tile {char: 'S', y: pos.0, x: pos.1, dir: Dir::Up};
    let mut tiles: Vec<Tile> = vec![first_tile];
    find_initial_path(&mut tiles, &pos, &mut chars);

    let mut next_path = tiles[1].clone();
    while next_path.char != 'S' {
        next_path = get_next_tile(&next_path, &mut chars);
        tiles.push(next_path.clone());
    }
    println!("Part 1: {}", tiles.len() / 2);
}

fn part2() {
    let input = include_str!("input");
    let mut chars = input
        .lines()
        .map(|str| str.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let pos = chars.iter().enumerate()
        .map(|(i, arr)| {
            arr.iter()
                .enumerate()
                .map(move |(j, c)| if c == &'S' { Some((i, j)) } else { None })
                .find(|x| x.is_some())
        })
        .flatten().next().unwrap().unwrap();
    let first_tile = Tile {char: 'S', y: pos.0, x: pos.1, dir: Dir::Up};
    let mut tiles: Vec<Tile> = vec![first_tile];
    find_initial_path(&mut tiles, &pos, &mut chars);

    let mut next_path = tiles[1].clone();
    while next_path.char != 'S' {
        next_path = get_next_tile(&next_path, &mut chars);
        tiles.push(next_path.clone());
    }
    let twice_area = get_area(&mut tiles);
    let inside_points = get_inside_points(twice_area, tiles.len() - 1);
    println!("{}", inside_points);
}

fn get_next_dir(char: char, dir: Dir) -> Dir {
    match char {
        'F' if dir == Dir::Left => Dir::Down,
        'F' => Dir::Right,
        'L' if dir == Dir::Left => Dir::Up,
        'L' => Dir::Right,
        '7' if dir == Dir::Right => Dir::Down,
        '7' => Dir::Left,
        'J' if dir == Dir::Right => Dir::Up,
        'J' => Dir::Left,
        _ => dir,
    }
}

fn find_initial_path(tiles: &mut Vec<Tile>, pos: &(usize, usize), chars: &mut Vec<Vec<char>>) {
    // If Starting tile is on an edge this function might panic from invalid index access
    let possible_coords = [(pos.0 - 1, pos.1), (pos.0, pos.1 + 1),
                            (pos.0 + 1, pos.1), (pos.0, pos.1 - 1)];
    let dirs = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];
    let (ind, coord) = possible_coords.iter().enumerate().find(|&(i, p)| 
                        connects(chars[p.0][p.1], &dirs[i])
                    ).unwrap();
    let char = chars[coord.0][coord.1];
    let dir = get_next_dir(char, dirs[ind].clone());
    let tile = Tile {char, y: coord.0, x: coord.1, dir};
    tiles.push(tile);
}

fn connects (char: char, dir: &Dir) -> bool {
    match dir {
        Dir::Up => char == '|' || char == 'F' || char == '7',
        Dir::Right => char == '-' || char == 'J' || char == '7',
        Dir::Down => char == '|' || char == 'J' || char == 'L',
        Dir::Left => char == '-' || char == 'F' || char == 'L',
    }
}

fn get_next_tile(tile: &Tile, chars: &mut Vec<Vec<char>>) -> Tile {
    let next_tile: Tile = match tile.dir {
        Dir::Up => {
            let dir = get_next_dir(chars[tile.y - 1][tile.x], tile.dir.clone());
            Tile { char: chars[tile.y - 1][tile.x], y: tile.y - 1, x: tile.x, dir}
        }
        Dir::Down => {
            let dir = get_next_dir(chars[tile.y + 1][tile.x], tile.dir.clone());
            Tile { char: chars[tile.y + 1][tile.x], y: tile.y + 1, x: tile.x, dir}
        }
        Dir::Right => {
            let dir = get_next_dir(chars[tile.y][tile.x + 1], tile.dir.clone());
            Tile { char: chars[tile.y][tile.x + 1], y: tile.y, x: tile.x + 1, dir}
        }
        Dir::Left => {
            let dir = get_next_dir(chars[tile.y][tile.x - 1], tile.dir.clone());
            Tile { char: chars[tile.y][tile.x - 1], y: tile.y, x: tile.x - 1, dir}
        }
    };
    next_tile
}

fn get_area(paths: &mut Vec<Tile>) -> isize {
    let sum: isize = paths.windows(2).map(|sl| (sl[1].y as isize * sl[0].x as isize) - (sl[1].x * sl[0].y) as isize).sum(); 
    let area = sum.abs();
    area
}
fn get_inside_points(twice_area: isize, len: usize) -> isize {
    let inside = (twice_area - len as isize) / 2 + 1;
    inside
}

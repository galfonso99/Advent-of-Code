use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
pub fn main() {
    // part1();
    part2()

    // Read the input into lines and collect into Vec<String>
    // Fo each element of the Vec do string.chars() and filter for chars that are =,*,#, etc
    // Then map the filtered symbols and for each find the numbers around it and diagonal you need
    // to change Vec index or string index or both
    // Then sum all those surrounding nums and return it for the map
    // Then sum all those values [.map().sum()]
    // At this point we should have a sum of the sum of the surrounding numbers for each symbol,
    // and we are done
    //
    // Actually check top down right left, then only check top corners if top was NAN and only
    // check bottom corners if bottom is a non-number
}

fn part1() {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().filter_map(|s| s.ok()).collect();
    let sum: usize = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .filter(|(_, c)| !c.is_digit(10) && c != &'.')
                .map(|(i, _)| {
                    let sum = look_around(y, i, &lines);
                    return sum;
                })
                .sum::<usize>()
        })
        .sum();
    println!("{}", sum);
}

fn part2() {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().filter_map(|s| s.ok()).collect();
    let sum: usize = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .filter(|(_, c)| !c.is_digit(10) && c != &'.')
                .map(|(i, _)| {
                    let sum = look_around_2(y, i, &lines);
                    return sum;
                })
                .sum::<usize>()
        })
        .sum();
    println!("{}", sum);
}

fn look_around(y: usize, x: usize, lines: &[String]) -> usize {
    let mut sum = 0_usize;
    let mut top_is_num = false;
    let mut bottom_is_num = false;
    if x != lines[y].len() - 1 {
        // right
        if char_at(&lines[y][x + 1 .. x + 2]).is_digit(10) {
           let num = get_number_right(x+1, &lines[y]);
           sum += num;
        }
    }
    if x != 0 {
        //left
        if char_at(&lines[y][x-1..x]).is_digit(10) {
           let num = get_number_left(x-1, &lines[y]);
           sum += num;
        }
    }
    if y != 0 {
        //top
        if char_at(&lines[y-1][x..x+1]).is_digit(10) {
            top_is_num = true;
            let num = get_number_vertical(x, &lines[y-1]);
            sum += num;
        }
        // top left
        if !top_is_num && x != 0 
            && char_at(&lines[y-1][x-1..x]).is_digit(10) {
                let num = get_number_left(x-1, &lines[y-1]);
                sum += num;
        } 
        // top right
        if !top_is_num && x != lines[y-1].len() - 1 
            && char_at(&lines[y-1][x+1..x+2]).is_digit(10) {
                let num = get_number_right(x+1, &lines[y-1]);
                sum += num;
        } 
    }
    if y != lines.len() - 1 {
        //bottom
        if char_at(&lines[y+1][x..x+1]).is_digit(10) {
            bottom_is_num = true;
            let num = get_number_vertical(x, &lines[y+1]);
            sum += num;
        }
        // bottom left
        if !bottom_is_num && x != 0 
            && char_at(&lines[y+1][x-1..x]).is_digit(10) {
                let num = get_number_left(x-1, &lines[y+1]);
                sum += num;
        } 
        // bottom right
        if !bottom_is_num && x != lines[y+1].len() - 1 
            && char_at(&lines[y+1][x+1..x+2]).is_digit(10) {
                let num = get_number_right(x+1, &lines[y+1]);
                sum += num;
        } 
    }
    sum
}
fn look_around_2(y: usize, x: usize, lines: &[String]) -> usize {
    let mut gears: Vec<usize> = Vec::new();
    let mut top_is_num = false;
    let mut bottom_is_num = false;
    if x != lines[y].len() - 1 {
        // right
        if char_at(&lines[y][x + 1..x + 2]).is_digit(10) {
            let num = get_number_right(x + 1, &lines[y]);
            gears.push(num);
        }
    }
    if x != 0 {
        //left
        if char_at(&lines[y][x - 1..x]).is_digit(10) {
            let num = get_number_left(x - 1, &lines[y]);
            gears.push(num);
        }
    }
    if y != 0 {
        //top
        if char_at(&lines[y - 1][x..x + 1]).is_digit(10) {
            top_is_num = true;
            let num = get_number_vertical(x, &lines[y - 1]);
            gears.push(num);
        }
        // top left
        if !top_is_num && x != 0 && char_at(&lines[y - 1][x - 1..x]).is_digit(10) {
            let num = get_number_left(x - 1, &lines[y - 1]);
            gears.push(num);
        }
        // top right
        if !top_is_num
            && x != lines[y - 1].len() - 1
            && char_at(&lines[y - 1][x + 1..x + 2]).is_digit(10)
        {
            let num = get_number_right(x + 1, &lines[y - 1]);
            gears.push(num);
        }
    }
    if y != lines.len() - 1 {
        //bottom
        if char_at(&lines[y + 1][x..x + 1]).is_digit(10) {
            bottom_is_num = true;
            let num = get_number_vertical(x, &lines[y + 1]);
            gears.push(num);
        }
        // bottom left
        if !bottom_is_num && x != 0 && char_at(&lines[y + 1][x - 1..x]).is_digit(10) {
            let num = get_number_left(x - 1, &lines[y + 1]);
            gears.push(num);
        }
        // bottom right
        if !bottom_is_num
            && x != lines[y + 1].len() - 1
            && char_at(&lines[y + 1][x + 1..x + 2]).is_digit(10)
        {
            let num = get_number_right(x + 1, &lines[y + 1]);
            gears.push(num);
        }
    }
    if gears.len() != 2 {
        return 0;
    }
    gears[0] * gears[1]
}

fn char_at(str: &str) -> char {
    str.chars().next().unwrap()
}
fn get_number_right(start: usize, str: &str) -> usize {
    str[start..]
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap()
}
fn get_number_left(end: usize, str: &str) -> usize {
    str[0..=end]
        .chars()
        .rev()
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}
fn get_number_vertical(x: usize, str: &str) -> usize {
    let left_shift = str[0..=x]
        .chars()
        .rev()
        .position(|c| !c.is_digit(10))
        .map(|x| x - 1)
        .unwrap_or(0);
    str[x - left_shift..]
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap()
}

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// Thinking about a solution
// For each dot found invalidate the row for the y value and the column for the x value
// Then any row or column that is left not invalidated should be twice as far 
// Create an array for the rows in which you store the index of the row that is empty
// Also  and array for the columns with the index of the column
// Using the empty rows and columns figure out the updated position of these dots (galaxies) store
// in a vec
// To figure out the updated y position count the empty rows up to the current y position and add one
// extra value for each
// To figure out the updated x position count the empty column up to the current x position and add
// one extra value for each
// For each galaxy pair find the delta of the row plus the delta of the column (absolute value only)
// Then add all of them together 
pub fn main () {
    // part1();
    part2();
}

fn get_input() -> Vec<Vec<char>> {
    let input = include_str!("input");
    input
        .lines()
        .map(|str| str.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn part1 () {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);

    let sum: isize = buf_reader.lines().map(|line| {
        let string = line.expect("expected");
        let mut nums = string.split(' ').filter_map(|str| str.parse().ok()).collect::<Vec<isize>>();
        let mut last_nums = vec![nums[nums.len() - 1]];
        loop {
            if nums.iter().all(|&x| x == nums[0]) {
                break;
            }
            nums = nums.windows(2).map(|slc| slc[1] - slc[0]).collect::<Vec<isize>>();
            last_nums.push(nums[nums.len() - 1]);

        }
        last_nums.iter().sum::<isize>()
    }).sum();
    println!("{}", sum);
}

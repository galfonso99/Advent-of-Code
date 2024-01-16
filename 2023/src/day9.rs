use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn main () {
    // part1();
    part2();
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

fn part2 () {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);
    
    let sum: isize = buf_reader.lines().map(|line| {
        let string = line.expect("expected");
        let mut nums = string.split(' ').filter_map(|str| str.parse().ok()).collect::<Vec<isize>>();
        let mut first_nums = vec![nums[0]];
        loop {
            if nums.iter().all(|&x| x == nums[0]) {
                break;
            }
            nums = nums.windows(2).map(|slc| slc[1] - slc[0]).collect::<Vec<isize>>();
            first_nums.push(nums[0]);
        }
        let res = first_nums.into_iter().enumerate().reduce(|acc, (i, x)| if i % 2 == 1 {(1, acc.1 - x)} else {(i, acc.1 + x)} ).unwrap();
        res.1
    }).sum();
    println!("{}", sum);
}

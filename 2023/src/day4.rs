use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
pub fn main() {
    // part1();
    part2()

}

fn part1() {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);
    // let lines: Vec<String> = buf_reader.lines().filter_map(|s| s.ok()).collect();
    let sum: usize = buf_reader.lines().map(|line| {
        let string = line.expect("expected");
        let (untrimmed_winning_nums, my_nums) = string.split_once('|').unwrap();
        let winning_nums: String = untrimmed_winning_nums.chars().skip_while(|&c| c != ':').skip(2).collect();
        let winning_array = winning_nums.trim().split(' ').filter_map(|str| str.parse().ok()).collect::<Vec<u32>>();
        let count = my_nums.trim().split(' ').filter_map(|str| str.parse().ok()).filter(|x| winning_array.contains(x)).count();
        if count == 0 {0} else {1 << (count - 1)}
    }).sum();
    println!("{}", sum);
}

fn part2() {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().filter_map(|s| s.ok()).collect();
    let len = lines.len();
    let mut cards: Vec<usize> = vec![Default::default(); len];
    let sum: usize = lines.iter().enumerate().map(|(i, line)| {
        // let string = line.expect("expected");
        let (untrimmed_winning_nums, my_nums) = line.split_once('|').unwrap();
        let winning_nums: String = untrimmed_winning_nums.chars().skip_while(|&c| c != ':').skip(2).collect();
        let winning_array = winning_nums.trim().split(' ').filter_map(|str| str.parse().ok()).collect::<Vec<u32>>();
        let matches = my_nums.trim().split(' ').filter_map(|str| str.parse().ok()).filter(|x| winning_array.contains(x)).count();
        let copies = 1 + cards[i];
        for x in 1..=matches {
            if let Some(ele) = cards.get_mut(i + x) {
                *ele += copies;
            }
        }
        copies
    }).sum();
    println!("{}", sum);
}

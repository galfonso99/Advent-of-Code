
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

    let lines = buf_reader.lines().map(|line| {
        let string = line.expect("expected");
        let significant = string.split(':').skip(1).next().unwrap();
        let nums: Vec<&str> = significant.split(' ').collect();
        nums.iter().filter_map(|x| x.trim().parse().ok()).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>();
    let times = &lines[0];
    let dists = &lines[1];
    let mut delta = 0;
    let product: usize = (0..times.len()).map(|i| {
        let time = &times[i];
        let dist = &dists[i];
        let first: usize = (1..*time).find(|x: &usize| ((*time - x) * x) > *dist).unwrap_or(0);
        let last: usize = (1..*time).rfind(|x: &usize| ((*time - x) * x) > *dist).unwrap_or(*time);
        delta = last - first + 1;
        delta
    }).product();
    println!("{}", product);
}

fn part2 () {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);

    let lines = buf_reader.lines().map(|line| {
        let string = line.expect("expected");
        let significant = string.split(':').skip(1).next().unwrap();
        let nums: Vec<&str> = significant.split(' ').collect();
        nums.iter().map(|x| x.trim()).filter(|x| x.parse::<usize>().is_ok()).collect::<String>()
    }).collect::<Vec<String>>();
    let time_string = &lines[0];
    let dist_string = &lines[1];
    let time: usize = time_string.parse().unwrap();
    let dist: usize = dist_string.parse().unwrap();
    let mut delta = 0;
    // let product: usize = (0..times.len()).map(|i| {
    //     // let time = &times[i];
    //     // let dist = &dists[i];
    //     let first: usize = (1..*time).find(|x: &usize| ((*time - x) * x) > *dist).unwrap_or(0);
    //     let last: usize = (1..*time).rfind(|x: &usize| ((*time - x) * x) > *dist).unwrap_or(*time);
    //     delta = last - first + 1;
    //     delta
    // }).product();
    let first: usize = (1..time).find(|x: &usize| ((time - x) * x) > dist).unwrap_or(0);
    let last: usize = (1..time).rfind(|x: &usize| ((time - x) * x) > dist).unwrap_or(time);
    delta = last - first + 1;

    println!("{}", delta);
}

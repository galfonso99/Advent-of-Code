use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn main () {
    // part1();
    part2();
}

fn part1 () {
    // Store all the lines into an array of Strings
    // Loop the array and store the signature (i.e. "AAA") into a map that maps it to the index
    // Then loop the array again and using left/right directions choose the next signature by
    // looking up in the hashmap which index it maps to 
    // Keep going until you reach ZZZ
    // print yhe count of the steps you took
    let (dirs, nodes) = parse_input();
    let mut map = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        map.insert(&node.0, i);
    }
    let end = String::from("ZZZ");
    let mut i = 0;
    let mut steps = 0;
    let mut node_ind = map[&"AAA".to_string()];
    while nodes[node_ind].0 != end {
        if dirs[i] < 0 {
            node_ind = map[&nodes[node_ind].1];
        } else if dirs[i] > 0 {
            node_ind = map[&nodes[node_ind].2];
        }
        i = ( i + 1 ) % dirs.len();
        steps += 1;
    }
    println!("{}", steps);
}

fn parse_input() -> (Vec<i32>, Vec<(String, String, String)>){
    let file = File::open("input").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input).unwrap();

    let (dir_strs, nodes_strs) = input.split_once("\n\n").unwrap();
    let dirs = dir_strs.chars().map(|c| if c == 'L' {-1} else {1}).collect::<Vec<i32>>();
    let nodes = nodes_strs.lines().map(|line| {
        let (start, ends) = line.split_once('=').unwrap();
        let (left, right) = ends.trim().split_once(' ').unwrap();
        let start = start.trim().to_string();
        let (left, right) = (left.strip_prefix('(').unwrap().strip_suffix(',').unwrap(), 
                             right.strip_suffix(')').unwrap());
        (start, left.to_string(), right.to_string())
    }).collect::<Vec<(String, String, String)>>();
    (dirs, nodes)
}

fn part2 () {
    let (dirs, nodes) = parse_input();
    let mut map = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        map.insert(&node.0, i);
    }
    let mut node_inds = nodes.iter().map(|x| &x.0).filter(|str| str.ends_with('A'))
                                    .map(|str| (map[&str], 0)).collect::<Vec<( usize, usize )>>();
    let mut i = 0;
    let mut steps = 0;
    loop {
        for (ind, cycle) in node_inds.iter_mut() {
            let finished = (&nodes[*ind]).0.ends_with('Z');
            if finished && cycle == &0 {
                *cycle = steps;
            }
            if dirs[i] < 0 {
                *ind = map[&nodes[*ind].1];
            } else if dirs[i] > 0 {
                *ind = map[&nodes[*ind].2];
            }
        }
        i = ( i + 1 ) % dirs.len();
        steps += 1;
        let condition = node_inds.iter().all(|(_, c)| c > &0);
        if condition {break;}
    }
    let lcm = node_inds.iter().map(|x| x.1 as usize).reduce(|acc, c| {
        lcm(acc, c)
    }).unwrap();
    println!("{}", lcm);
}

fn lcm (mut a: usize, mut b: usize) -> usize {
    if a < b {
        (a, b) = (b, a);
    }
    ( a / hcd(a, b) ) * b
}

fn hcd (mut a: usize, mut b: usize) -> usize {
    if a < b {
        (a, b) = (b, a);
    }
    fn aux (a: usize, b: usize) -> usize {
        if b == 0 {return a;}
        aux(b, a % b)
    }
    aux(a, b)
}

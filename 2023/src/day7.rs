
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn main () {
    // part1();
    part2();
}

fn part1 () {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);

    let mut sorted_hands = buf_reader.lines().map(|line| {
        let string = line.expect("expected");
        let (hand, bid_str) = string.split_once(' ').unwrap();
        let bid: usize = bid_str.parse().unwrap();
        (hand.to_owned(), bid)
    }).collect::<Vec<(String, usize)>>();
    sorted_hands.sort_unstable_by(|(a, _) ,(b, _)| sort_them(a, b));
    let sum: usize = sorted_hands.iter().enumerate().map(|(i, (_, bid))| bid * (i + 1)).sum();
    println!("{}", sum);
}

fn sort_them (a: &str, b: &str) -> Ordering {
    let mut ordering = hand_type(a).cmp(&hand_type(b));
    if ordering == Ordering::Equal {
        ordering = second_ordering(a, b);
    }
    ordering
}
fn hand_type(hand: &str) -> u8 {
    let mut groups: Vec<usize> = vec![0; 14];
    hand.chars().for_each(|c| {
        let index = map(c); 
        groups[index] += 1;
    });
    groups.sort_by(|a, b| b.cmp(a));
    match groups[0] {
        5 => 7,
        4 => 6,
        3 => {
            match groups[1] {
                2 => 5,
                _ => 4,
            }
        },
        2 => {
            match groups[1] {
                2 => 3,
                _ => 2,
            }
        },
        1 => 1,
        _ => 1,
    }
}

fn map(c: char) -> usize {
    let map: HashMap<char, usize> = HashMap::from([('2', 1), ('3', 2), ('4', 3), ('5', 4), ('6', 5), ('7', 6), 
                            ('8', 7), ('9', 8), ('T', 9), ('J', 10), ('Q', 11), ('K', 12), ('A', 13)]);
    map[&c]
}

fn second_ordering(a: &str, b: &str) -> Ordering {
    // Create a hashmap where each card characted has a corresponding int value
    let (a, b) = a.chars().zip(b.chars()).skip_while(|(c1, c2)| c1 == c2).next().unwrap_or(('2', '3'));
    map(a).cmp(&map(b))
}

fn part2 () {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);

    let mut sorted_hands = buf_reader.lines().map(|line| {
        let string = line.expect("expected");
        let (hand, bid_str) = string.split_once(' ').unwrap();
        let bid: usize = bid_str.parse().unwrap();
        (hand.to_owned(), bid)
    }).collect::<Vec<(String, usize)>>();
    sorted_hands.sort_unstable_by(|(a, _) ,(b, _)| sort_them_2(a, b));
    let sum: usize = sorted_hands.iter().enumerate().map(|(i, (_, bid))| bid * (i + 1)).sum();
    println!("{}", sum);
}
fn sort_them_2 (a: &str, b: &str) -> Ordering {
    let mut ordering = hand_type_2(a).cmp(&hand_type_2(b));
    if ordering == Ordering::Equal {
        ordering = second_ordering_2(a, b);
    }
    ordering
}

fn hand_type_2(hand: &str) -> u8 {
    let mut groups: Vec<usize> = vec![0; 14];
    let mut j_count = 0;
    hand.chars().for_each(|c| {
        if c == 'J' {
            j_count += 1;
        } else {
            let index = map_2(c); 
            groups[index] += 1;
        }
    });
    groups.sort_by(|a, b| b.cmp(a));
    groups[0] += j_count;
    match groups[0] {
        5 => 7,
        4 => 6,
        3 => {
            match groups[1] {
                2 => 5,
                _ => 4,
            }
        },
        2 => {
            match groups[1] {
                2 => 3,
                _ => 2,
            }
        },
        1 => 1,
        _ => 1,
    }
}
fn map_2(c: char) -> usize {
    let map: HashMap<char, usize> = HashMap::from([('J', 0), ('2', 1), ('3', 2), ('4', 3), ('5', 4), ('6', 5), ('7', 6), 
                            ('8', 7), ('9', 8), ('T', 9), ('Q', 10), ('K', 11), ('A', 12)]);
    map[&c]
}

fn second_ordering_2(a: &str, b: &str) -> Ordering {
    // Create a hashmap where each card characted has a corresponding int value
    let (a, b) = a.chars().zip(b.chars()).skip_while(|(c1, c2)| c1 == c2).next().unwrap_or(('2', '3'));
    map_2(a).cmp(&map_2(b))
}

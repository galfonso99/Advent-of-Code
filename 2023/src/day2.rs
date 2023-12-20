use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const BLUE_MAX: usize = 14;
const RED_MAX: usize = 12;
const GREEN_MAX: usize = 13;

pub fn main () {
    // part1();
    part2();
}

fn part1 () {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);

    let sum = buf_reader.lines().map(|line| {
        let string = line.expect("expected");
        let mut game_id = string[5..].chars().take_while(|&c| c != ':').collect::<String>()
                    .parse::<usize>().unwrap_or(0);
        let game = string.split(':').skip(1).next().unwrap();
        let sets: Vec<&str> = game.split(';').collect();
        for ele in sets {
            let dice = ele.split(',').collect::<Vec<&str>>();
            for x in dice {
                let die = x.trim().split(' ').collect::<Vec<&str>>();
                let (num, color) = (die[0].parse::<usize>().unwrap_or(0), die[1]);
                match color {
                    "blue" => if num > BLUE_MAX {game_id = 0},
                    "red" => if num > RED_MAX {game_id = 0},
                    "green" => if num > GREEN_MAX {game_id = 0},
                    _ => (),
                }
            }
        }
        return game_id;
    }).sum::<usize>();
    println!("{}", sum);
}

fn part2 () {
    use std::collections::HashMap;
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);

    let sum = buf_reader.lines().map(|line| {
        let string = line.expect("expected");
        let mut max_map: HashMap<&str, usize> = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
        let game = string.split(':').skip(1).next().unwrap();
        let sets: Vec<&str> = game.split(';').collect();
        for ele in sets {
            let dice = ele.split(',').collect::<Vec<&str>>();
            for x in dice {
                let die = x.trim().split(' ').collect::<Vec<&str>>();
                let (num, color) = (die[0].parse::<usize>().unwrap_or(0), die[1]);
                if num > max_map[color] {
                    max_map.insert(color, num);
                }
            }
        }
        let power = max_map["green"] * max_map["red"] * max_map["blue"];
        return power;
    }).sum::<usize>();
    println!("{}", sum);
}

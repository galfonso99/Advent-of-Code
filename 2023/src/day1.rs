use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn main() {
    // part1();
    part2();
}
const NUMS: &[&str] = &["\n", "one", "two", "three","four","five","six","seven","eight", "nine"];

fn part1() {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);
    let int_arrays = buf_reader.lines().map(|line| 
            line.expect("Expected").chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    let sum: u32 = int_arrays.iter().map(|arr| arr[0] * 10 + arr[arr.len()-1]).sum();
    println!("{:?}", sum);

    /* Reading the buffer into a string and then process it */
    // let mut contents = String::new();
    // buf_reader.read_to_string(&mut contents).unwrap();
    // let int_arrays = contents.lines().map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>())
    //                     .collect::<vec<vec<u32>>>();
}

fn part2() {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);
    let sum: u32 = buf_reader.lines().map(|line| { 
            let line = line.expect("expected"); 
            let first = find_first_digit(&line);
            let last = find_last_digit(&line);
            first * 10 + last
    }).sum();
    println!("{:?}", sum);
}

fn find_first_digit(str: &String) -> u32 {
    str.char_indices().find_map(|(i,c)| {
        let char = c.to_digit(10).map(|x| x as usize);
        let named_char = NUMS.iter().position(|s| str[i..].starts_with(s));
        char.or(named_char)
    }).unwrap() as u32
}
fn find_last_digit(str: &String) -> u32 {
    str.char_indices().rev().find_map(|(i,c)| {
        let char = c.to_digit(10).map(|x| x as usize);
        let named_char = NUMS.iter().position(|s| str[i..].starts_with(s));
        char.or(named_char)
    }).unwrap() as u32
}

// fn part2() {
//     let file = File::open("input").unwrap();
//     let buf_reader = BufReader::new(file);
//     let int_arrays = buf_reader.lines().map(|line| 
//             line.expect("Expected").replace("one", "o1e").replace("two", "t2o").replace("three", "t3e").replace("four", "fo4r")
//                  .replace("five", "fi5e").replace("six", "s6x").replace("seven", "sev7n").replace("eight", "eig8t").replace("nine", "n9ne")
//             .chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>())
//         .collect::<Vec<Vec<u32>>>();
//     let sum: u32 = int_arrays.iter().map(|arr| arr[0] * 10 + arr[arr.len()-1]).sum();
//     println!("{:?}", sum);
// }

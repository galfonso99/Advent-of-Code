use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
pub fn main() {
    // part1();
    part2()

}
fn part1() {
    let input = parse_input();
    let seeds = input[0].trim().split(' ').filter_map(|str| str.parse().ok()).collect::<Vec<usize>>();
    let soil_map = create_map(&input[1]);
    let fertilizer_map = create_map(&input[2]);
    let water_map = create_map(&input[3]);
    let light_map = create_map(&input[4]);
    let temperature_map = create_map(&input[5]);
    let humidity_map = create_map(&input[6]);
    let location_map = create_map(&input[7]);
    let min = seeds.iter().map(|&seed| {
        let change_1 = map(seed, &soil_map);
        let change_2 = map(change_1, &fertilizer_map);
        let change_3 = map(change_2, &water_map);
        let change_4 = map(change_3, &light_map);
        let change_5 = map(change_4, &temperature_map);
        let change_6 = map(change_5, &humidity_map);
        let location = map(change_6, &location_map);
        location
    }).min().unwrap();
    println!("{}", min);
}

fn parse_input () -> Vec<String> {
    // Returna Vec of strings corresponding to seeds and maps
    let file = File::open("input").unwrap();
    let mut buf_reader = BufReader::new(file);
    // let lines: Vec<String> = buf_reader..split().lines().filter_map(|s| s.ok()).collect();
    let mut input = String::new();
    buf_reader.read_to_string(&mut input).unwrap();

    input.split("\n\n").enumerate().map(|(i, str)| {
        if i == 0 {return str.split(':').skip(1).next().unwrap().to_owned()} 
        str.lines().skip(1).intersperse("\n").collect::<String>()
    }).collect::<Vec<String>>()
}

fn create_map(map_string: &str) -> HashMap<usize, (usize, usize)> { 
    let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
    map.insert(0, (0, 1));    // establishing lowest point
    // parse the string 
    for range in map_string.lines() {
        let nums: Vec<usize> = range.trim().split(' ').filter_map(|str| str.parse().ok()).collect();
        let (dest, init, range) = (nums[0], nums[1], nums[2]);
        map.insert(init, (dest, range));
    }
    map
}
fn map (num: usize, map: &HashMap<usize, (usize, usize)>) -> usize {
    let mut lowest = 0;
    for &key in map.keys() {
        if key <= num && key > lowest {
            lowest = key;
        }
    }
    let (dest, range) = map[&lowest];
    if num <= lowest + range - 1 {
        let delta = num - lowest;
        dest + delta
    } else {
        num
    }
}
fn part2() {
    let input = parse_input();
    let seeds = input[0].trim().split(' ').filter_map(|str| str.parse().ok()).collect::<Vec<usize>>();
    let soil_map = create_map_2(&input[1]);
    let fertilizer_map = create_map_2(&input[2]);
    let water_map = create_map_2(&input[3]);
    let light_map = create_map_2(&input[4]);
    let temperature_map = create_map_2(&input[5]);
    let humidity_map = create_map_2(&input[6]);
    let location_map = create_map_2(&input[7]);
    let min = seeds.chunks(2).map(|chk| {
        let soil = map_2(vec![(chk[0], chk[0] + chk[1] - 1)], &soil_map);
        let fertilizer = map_2(soil, &fertilizer_map);
        let water = map_2(fertilizer, &water_map);
        let light = map_2(water, &light_map);
        let temperature = map_2(light, &temperature_map);
        let humidity = map_2(temperature, &humidity_map);
        let location = map_2(humidity, &location_map);
        location.iter().map(|x| x.0).collect::<Vec<_>>()
    }).flatten().min().unwrap();
    println!("{}", min);
}

fn create_map_2(map_string: &str) -> Vec<(usize, usize, usize)> { 
    // parse the string 
    let mut map: Vec<(usize, usize, usize)> = map_string.lines().map(|range| {
        let nums: Vec<usize> = range.trim().split(' ').filter_map(|str| str.parse().ok()).collect();
        let (dest, init, range) = (nums[0], nums[1], nums[2]);
        (init, init+range-1, dest)
    }).collect();
    map.sort_by_key(|x| x.0);
    map.push((usize::MAX, usize::MAX, usize::MAX));
    if map[0].0 > 0 {
        map.push((0, map[0].0 - 1, 0));
    }
    map.sort_by_key(|x| x.0);
    map
}
fn map_2 (ranges: Vec<(usize, usize)>, map: &Vec<(usize, usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_ranges: Vec<(usize, usize)> = Vec::new();
    for range in ranges.iter() {
        let mut i = range.0;
        while i <= range.1 {
            let upper_ind = map.iter().position(|(a, _, _)| a > &i).unwrap();
            let lower_ind = upper_ind - 1;
            let my_range = map[lower_ind];
            let upper_range = map[upper_ind];
            if my_range.0 <= i && i <= my_range.1 {
                let matching_range = (i, range.1.min(my_range.1));
                let delta: i64 = my_range.2 as i64 - my_range.0 as i64; 
                let mapped_range = ( (matching_range.0 as i64 + delta) as usize, 
                                    (matching_range.1 as i64 + delta) as usize );
                new_ranges.push(mapped_range);
                i = matching_range.1;
            } else {
                let mapped_range = (i, (upper_range.0 - 1).min(range.1));
                new_ranges.push(mapped_range);
                i = mapped_range.1;
            }
            i += 1;
        }
    }
    new_ranges
}

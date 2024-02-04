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

fn part1 () {
    let input = get_input();
    let mut galaxies = vec![];
    let mut rows = vec![false; input.len()];
    let mut cols = vec![false; input[0].len()];
    input.iter().enumerate().for_each(|(i, row)| {
            (0..input[i].len()).for_each(|j| {
                    if row[j] != '#' {return;}
                    rows[i] = true;
                    cols[j] = true;
                    galaxies.push((i as isize, j as isize));
            });
    });
    let mut rows_count = 0;
    let mut cols_count = 0;
    let rows = rows.iter().map(|&occupied| {
        if !occupied {rows_count += 1;}
        rows_count
    }).collect::<Vec<isize>>();
    let cols = cols.iter().map(|&occupied| {
        if !occupied {cols_count += 1;}
        cols_count
    }).collect::<Vec<isize>>();
    galaxies = galaxies.iter().map(|&(i, j)| (i + rows[i as usize], j + cols[j as usize]))
                    .collect::<Vec<(isize, isize)>>();
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let delta = (galaxies[j].0 - galaxies[i].0).abs() + (galaxies[j].1 - galaxies[i].1).abs();
            sum += delta;
        }
    }
    // println!("{:?}", rows);
    // println!("{:?}", cols);
    println!("{:?}", sum);
}

// Part 2 is the same as part 1 but changing by how much each distance increases
fn part2 () {
    let input = get_input();
    let mut galaxies = vec![];
    let mut rows = vec![false; input.len()];
    let mut cols = vec![false; input[0].len()];
    input.iter().enumerate().for_each(|(i, row)| {
            (0..input[i].len()).for_each(|j| {
                    if row[j] != '#' {return;}
                    rows[i] = true;
                    cols[j] = true;
                    galaxies.push((i as isize, j as isize));
            });
    });
    let mut rows_count = 0;
    let mut cols_count = 0;
    let rows = rows.iter().map(|&occupied| {
        if !occupied {rows_count += 999_999;}
        rows_count
    }).collect::<Vec<isize>>();
    let cols = cols.iter().map(|&occupied| {
        if !occupied {cols_count += 999_999;}
        cols_count
    }).collect::<Vec<isize>>();
    galaxies = galaxies.iter().map(|&(i, j)| (i + rows[i as usize], j + cols[j as usize]))
                    .collect::<Vec<(isize, isize)>>();
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let delta = (galaxies[j].0 - galaxies[i].0).abs() + (galaxies[j].1 - galaxies[i].1).abs();
            sum += delta;
        }
    }
    println!("{:?}", sum);
}

fn get_input() -> Vec<Vec<char>> {
    let input = include_str!("input");
    input
        .lines()
        .map(|str| str.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

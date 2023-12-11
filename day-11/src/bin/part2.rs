use std::fs;
use std::cmp;

fn solve(file_path: &str) -> u64 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut sum = 0;
    
    let mut image: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        image.push(line.chars().collect());

    }

    // insert rows
    for i in 0..image.len() {
        if image[i].iter().all(|&x| x == '.') {
            image[i] = image[i].iter().map(|_| '*').collect();
        }
    }

    // insert columns
    for i in 0..image[0].len() {
        if image.iter().all(|x| x[i] == '.' || x[i] == '*') {
            for j in 0..image.len() {
                image[j][i] = '*';
            }
        }
    }

    // find all #'s
    let mut coordinates: Vec<(i32, i32)> = Vec::new();
    for i in 0..image.len() {
        for j in 0..image[0].len() {
            if image[i][j] == '#' {
                coordinates.push((i as i32, j as i32));
            }
        }
    }
   
    for i in 0..coordinates.len() {
        for j in i+1..coordinates.len() {
            let mut d1:u64 = (coordinates[i].0 - coordinates[j].0).abs() as u64;
            for k in 1..d1 {
                if image[cmp::min(coordinates[i].0, coordinates[j].0) as usize + k as usize][coordinates[i].1 as usize] == '*' {
                    d1 += 1000000 - 1;
                }
            }
            let mut d2: u64 = (coordinates[i].1 - coordinates[j].1).abs() as u64;
            for k in 1..d2 {
                if image[coordinates[i].0 as usize][cmp::min(coordinates[i].1, coordinates[j].1) as usize + k as usize] == '*' {
                    d2 += 1000000 - 1;
                }
            }
            sum += d1 + d2;
        }
    }
    return sum;
}

fn main() {
    let  sum = solve("./src/bin/part1_input.txt");
    println!("{}", sum);
}



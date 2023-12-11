use std::fs;

fn solve(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut sum = 0;
    
    let mut image: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        image.push(line.chars().collect());

    }

    // insert rows
    let mut count_inserted_rows = 0;
    for i in 0..image.len() {
        if image[i + count_inserted_rows].iter().all(|&x| x == '.') {
            image.insert(i + count_inserted_rows, image[i + count_inserted_rows].clone());
            count_inserted_rows += 1;
        }
    }

    // insert columns
    let mut count_inserted_cols = 0;
    for i in 0..image[0].len() {
        if image.iter().all(|x| x[i + count_inserted_cols] == '.') {
            for j in 0..image.len() {
                image[j].insert(i + count_inserted_cols, '.');
            }
            count_inserted_cols += 1;
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
            let d1 = (coordinates[i].0 - coordinates[j].0).abs();
            let d2 = (coordinates[i].1 - coordinates[j].1).abs();
            sum += d1 + d2;
        }
    }
    return sum;
}

fn main() {
    let  sum = solve("./src/bin/part1_input.txt");
    println!("{}", sum);
}



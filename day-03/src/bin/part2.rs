use std::fs;

fn is_star(c: char) -> bool {
    return c == '*'; 
}

fn check_neighbours(coordinate_i:usize, coordinate_j:usize, arr:&Vec<Vec<char>>)  -> (isize, isize) {
    for i in [-1, 0, 1] {
        if (i + coordinate_i as isize > -1) && ((i + coordinate_i as isize)  < (arr.len() as isize)) {
            for j in [-1, 0, 1] {
                if (j + coordinate_j as isize > -1) && ((j + coordinate_j as isize)  < (arr[(i + coordinate_i as isize) as usize].len() as isize)) && 
                is_star(arr[(i + coordinate_i as isize) as usize][(j + coordinate_j as isize) as usize]) {
                    return (i + coordinate_i as isize, j + coordinate_j as isize);
                }
            }
        }
    }
    return (-1, -1);
}

fn solve(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut sum = 0;
    
    let mut chars: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
       chars.push(line.chars().collect());
    }

    let mut ratios: Vec<Vec<(u32, u32)>> = Vec::new();
    for i in 0..chars.len() {
        ratios.push(Vec::new());
        for _j in 0..chars[0].len() {
            ratios[i].push((0,0));
        }
    }

    let mut current_number: u32 = 0;
    let mut star_coordinates = (-1,-1);

    for (i, line) in chars.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                current_number *= 10;
                current_number += c.to_digit(10).unwrap();
                if star_coordinates == (-1, -1) {
                    star_coordinates = check_neighbours(i, j, &chars);
                }
            }   else if star_coordinates != (-1, -1) || j == chars[0].len()
            {
                if ratios[star_coordinates.0 as usize][star_coordinates.1 as usize].0 == 0 {
                    ratios[star_coordinates.0 as usize][star_coordinates.1 as usize].0 = current_number;
                } else {
                    ratios[star_coordinates.0 as usize][star_coordinates.1 as usize].1 = current_number;
                }
                
                current_number = 0;
                star_coordinates = (-1, -1);
            } 
            else {
                current_number = 0;
            }
        }
        if star_coordinates != (-1, -1)
        {
            if ratios[star_coordinates.0 as usize][star_coordinates.1 as usize].0 == 0 {
                ratios[star_coordinates.0 as usize][star_coordinates.1 as usize].0 = current_number;
            } else {
                ratios[star_coordinates.0 as usize][star_coordinates.1 as usize].1 = current_number;
            }
            
        }
        star_coordinates = (-1, -1);
        current_number = 0;
    }
    for i in 0..chars.len() {
        for j in 0..chars[0].len() {
            sum += ratios[i][j].0 * ratios[i][j].1;
        }
    } 

    return sum;
}

fn main() {
    let  sum = solve("./src/bin/part2_input.txt");
    println!("{}", sum);
}
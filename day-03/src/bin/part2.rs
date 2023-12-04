use std::fs;

fn is_not_digit_or_dot(c: char) -> bool {
    return !c.is_digit(10) && c != '.'; 
}

fn check_neighbours(coordinate_i:usize, coordinate_j:usize, arr:&Vec<Vec<char>>)  -> bool {
    for i in [-1, 0, 1] {
        if (i + coordinate_i as isize > -1) && ((i + coordinate_i as isize)  < (arr.len() as isize)) {
            for j in [-1, 0, 1] {
                if (j + coordinate_j as isize > -1) && ((j + coordinate_j as isize)  < (arr[(i + coordinate_i as isize) as usize].len() as isize)) && 
                is_not_digit_or_dot(arr[(i + coordinate_i as isize) as usize][(j + coordinate_j as isize) as usize]) {
                    return true;
                }
    
            }
        }
        
    }
    return false;
}

fn solve(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut sum = 0;
    
    let mut chars: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
       chars.push(line.chars().collect());
    }

    let mut current_number: u32 = 0;
    let mut current_number_has_char = false;

    for (i, line) in chars.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if (c.is_digit(10)) {
                current_number *= 10;
                current_number += c.to_digit(10).unwrap();
                current_number_has_char |= check_neighbours(i, j, &chars);
            }   else 
            {
                if current_number_has_char {
                    sum += current_number;
                }
                else if current_number != 0{
                }
                current_number = 0;
                current_number_has_char = false;
            }
        }
        if current_number_has_char {
            sum += current_number;
        }
        else if current_number != 0{
        }
        current_number = 0;
        current_number_has_char = false;
    }

    return sum;
}

fn main() {
    let  sum = solve("./src/bin/part2_input.txt");
    println!("{}", sum);
}

#[cfg(test)]
#[test]
fn example_works() {
    assert_eq!(solve("./src/bin/part2_example.txt"), 467835);
}


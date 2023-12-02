use std::fs;

fn solve(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut sum = 0;
    
    for line in input.lines() {
        let all_numbers = line.rmatch_indices(char::is_numeric);
        let (_, last_digit) = all_numbers.clone().nth(0).unwrap();
        let (_, first_digit) = all_numbers.clone().last().unwrap();
        sum += format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
    }

    return sum;
}

fn main() {
    let  sum = solve("./src/bin/part1_input.txt");
    println!("{}", sum);
}

#[cfg(test)]
#[test]
fn example_works() {
    assert_eq!(solve("./src/bin/part1_example.txt"), 142);
}


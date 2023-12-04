use std::{fs};

fn solve(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut sum = 0;
    
    for line in input.lines() {
        let cards = line.split(": ").collect::<Vec<&str>>().into_iter().nth(1).unwrap();
        let winning:Vec<i32> = cards.split(" | ")
            .collect::<Vec<&str>>()
            .into_iter().nth(0).unwrap()
            .split(" ").collect::<Vec<&str>>().into_iter()
            .filter(|&x| !x.is_empty())
            .map(|x|->i32{x.parse().unwrap()}).collect();
        let available :Vec<i32> = cards.split(" | ")
            .collect::<Vec<&str>>()
            .into_iter().nth(1).unwrap()
            .split(" ").collect::<Vec<&str>>().into_iter()
            .filter(|&x| !x.is_empty())
            .map(|x|->i32{x.parse().unwrap()}).collect();

        let mut i = 0;
        for card in available {
            if winning.contains(&card) {
                if i == 0 {
                    i = 1;
                } else {
                    i *= 2;
                }
            }
        }
        sum += i;
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
    assert_eq!(solve("./src/bin/part1_example.txt"), 13);
}

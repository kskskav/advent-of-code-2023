use std::{fs};

fn solve(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut sum = input.clone().lines().count() as i32;

    let mut count_cards: Vec<i32> = Vec::new();
    for _i in 1..sum+1 {
        count_cards.push(1);
    }
    sum = 0;

    for (i, line) in input.lines().into_iter().enumerate() {
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
        let mut current_count_winning = 0;
        for card in available {
            if winning.contains(&card) {
                current_count_winning += 1;
            }
        }
        for j in i+1..i+1+current_count_winning {
            count_cards[j] += count_cards[i]; 
        }
    }
   
   for i in count_cards {
    sum += i;
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
    assert_eq!(solve("./src/bin/part2_example.txt"), 30);
}


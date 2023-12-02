use std::fs;
use std::collections::HashMap;

fn replace_words_with_digit(s: &str) -> String {
    let mut numbers : HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let (mut first_word, mut first_index): (&str, usize) = ("", usize::MAX);
    let (mut last_word, mut last_index): (&str, usize) = ("", usize::MIN);

    for (word, digit) in numbers.clone() {
        let last = s.rfind(word).unwrap_or_else(|| usize::MIN);
        let first = s.find(word).unwrap_or_else(|| usize::MAX);
        if first < first_index {
            first_index = first;
            first_word = word;
        }
        if last > last_index {
            last_index = last;
            last_word = word;
        }
        
    }
    let mut res = s.clone().to_string();
    if last_index != usize::MIN {
        res.replace_range(last_index..last_index, numbers[&last_word]);
    }
    if first_index != usize::MAX {
        res.replace_range(first_index..first_index, numbers[&first_word]);
    }
    println!("{res}");
    return res;
}

fn solve(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let corrected_line = replace_words_with_digit(line);
        let all_numbers = corrected_line.rmatch_indices(char::is_numeric);
        let (_, last_digit) = all_numbers.clone().nth(0).unwrap();
        let (_, first_digit) = all_numbers.clone().last().unwrap();
        sum += format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
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
    assert_eq!(solve("./src/bin/part2_example.txt"), 281);
}


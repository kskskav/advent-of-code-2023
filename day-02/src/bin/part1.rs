use std::fs;

struct Sample {
    red: i32,
    green: i32,
    blue: i32,
}

impl Sample {
    // s in form "8 green, 6 blue, 20 red"
    pub fn parse(&mut self, s: &String ) {
        *self = Sample {red: 0, green: 0, blue: 0};
        for color in s.split(", ").collect::<Vec<&str>>() {
            if color.contains("red") {
                self.red = color.split(" ").collect::<Vec<&str>>().into_iter().nth(0).unwrap().parse().unwrap();
            }
            else if color.contains("green") {
                self.green = color.split(" ").collect::<Vec<&str>>().into_iter().nth(0).unwrap().parse().unwrap();
            }
            else if color.contains("blue") {
                self.blue = color.split(" ").collect::<Vec<&str>>().into_iter().nth(0).unwrap().parse().unwrap();
            }
        }
    }

    pub fn validate(&self) -> bool {
        return self.red < 13 && self.green < 14 && self.blue < 15;
    }

    pub fn print(&self) {
        println!("red: {}, green: {}, blue: {};", self.red, self.green, self.blue);
    }
}

fn solve(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut sum = 0;
    
    let mut i = 1;
    for line in input.lines() {
        let mut game_valid = true;
        let mut sample = Sample {red: 0, green: 0, blue: 0};
        let samples_str = line.split(": ").collect::<Vec<&str>>().into_iter().nth(1).unwrap();
        
        for sample_str in samples_str.split("; ").collect::<Vec<&str>>() {
            sample.parse(&String::from(sample_str));
            game_valid &= sample.validate();
        }
        if game_valid {
            sum += i;
        }
        i +=1;
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
    assert_eq!(solve("./src/bin/part1_example.txt"), 8);
}


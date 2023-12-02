use std::{fs, cmp::max};

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

    pub fn update_min(&mut self, other: &Sample) {
        self.red = max(self.red, other.red);
        self.green = max(self.green, other.green);    
        self.blue = max(self.blue, other.blue);
    }

    pub fn print(&self) {
        println!("red: {}, green: {}, blue: {};", self.red, self.green, self.blue);
    }

    pub fn power(&self)  -> i32 {
        return self.red * self.blue * self.green;
    }
}

fn solve(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut sum = 0;
    
    for line in input.lines() {
        let mut sample = Sample {red: 0, green: 0, blue: 0};
        let mut minimum = Sample {red: 1, green: 1, blue: 1};
        let samples_str = line.split(": ").collect::<Vec<&str>>().into_iter().nth(1).unwrap();
        
        for sample_str in samples_str.split("; ").collect::<Vec<&str>>() {
            sample.parse(&String::from(sample_str));
            minimum.update_min(&sample);
        }
        
        sum += minimum.power();
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
    assert_eq!(solve("./src/bin/part2_example.txt"), 2286);
}


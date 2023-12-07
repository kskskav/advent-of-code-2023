use std::{fs, cmp::Ordering, collections::HashMap};

fn solve(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut res: i32 = 0;
    let mut hands = Vec::<Hand>::new();
    for line in input.lines() {
        hands.push(Hand::new(String::from(line)));
    }
    hands.sort_by(|a, b| a.cmp(b));
    for (i, elem) in hands.iter().enumerate() {
        res += (i as i32 +1) * elem.bid;
    }
    return res;
}


fn main() {
    let  sum = solve("./src/bin/part2_input.txt");
    println!("{}", sum);
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Type {
    FIVE,
    FOUR,
    FULL,
    THREE,
    TWO,
    ONE,
    HIGH
}
impl Type {
    fn cmp(a: &Type, b: &Type) -> Ordering {
        return (TYPE_ORDER.iter().position(|x| x == a)).cmp(&TYPE_ORDER.iter().position(|x| x == b));
    }
}

#[derive(Debug)]
struct Hand {
    hand: String,
    bid: i32,
    typ: Type,
}

const TYPE_ORDER: [Type; 7] = [Type::HIGH, Type::ONE, Type::TWO, Type::THREE, Type::FULL, Type::FOUR, Type::FIVE];
const CHAR_ORDER: [char; 14] = ['J','2','3','4','5','6','7','8','9','T','J','Q','K','A'];


impl Hand {
    pub fn new(input: String) -> Hand {
        let hand = String::from(input.split_whitespace().next().unwrap());
        let bid:i32 = input.split_whitespace().last().unwrap().parse().unwrap();

        let mut letter_counts: HashMap<char,i32> = HashMap::new();

        let char_vec: Vec<char> = hand.chars().collect();
        for c in char_vec {
            *letter_counts.entry(c).or_insert(0) += 1;
        }
        let mut typ = Type::ONE;

        let j_count = if letter_counts.get(&'J').is_some() {*letter_counts.get(&'J').unwrap()} else {0};
        letter_counts.remove(&'J');

        if j_count == 5 {
            typ = Type::FIVE;
        }
        else if letter_counts.values().max().unwrap() + j_count == 5 {
            typ = Type::FIVE;
        }
        else if letter_counts.values().max().unwrap() + j_count == 4 {
            typ = Type::FOUR;
        }
        else if letter_counts.values().any(|&x| x == 2) && letter_counts.values().any(|&x| x == 3) 
            || j_count == 1 && letter_counts.values().filter(|&&x| x == 2).count() == 2 {
            typ = Type::FULL;
        }
        else if letter_counts.values().max().unwrap() + j_count == 3 {
            typ = Type::THREE;
        }
        else if letter_counts.values().filter(|&&x| x == 2).count() == 2 
            || (j_count == 1 && letter_counts.values().any(|&x| x == 2))
            || j_count == 2 {
            typ = Type::TWO;
        }else if letter_counts.values().any(|&x| x == 2) || j_count != 0 {
            typ = Type::ONE;
        }
        else {
            typ = Type::HIGH;
        }

        return Hand { hand, bid, typ };
    }

    fn cmp_chars(a: char, b: char) -> Ordering {
        return (CHAR_ORDER.iter().position(|&x| x == a)).cmp(&CHAR_ORDER.iter().position(|&x| x == b));
    }

    pub fn cmp(&self, other: &Hand) ->Ordering {
        if self.typ != other.typ {
            return Type::cmp(&self.typ, &other.typ);
        } else {
            for (i, a) in self.hand.chars().into_iter().enumerate() {
                let res = Hand::cmp_chars(a, other.hand.chars().nth(i).unwrap());
                if  res != Ordering::Equal {
                    return res;
                };
            }
            return Ordering::Equal;
        }
    }
}
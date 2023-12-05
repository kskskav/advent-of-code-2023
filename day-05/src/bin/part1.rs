use std::fs;
use std::collections::HashMap;
use sortby::SortByIteratorExt;

fn solve(file_path: &str) -> i64 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut minimum = i64::MAX;
    
    let mut mapper = Mapper {seeds: Vec::new(), 
        seed_to_soil: HashMap::new(),
        soil_to_fertilizer: HashMap::new(),
        fertilizer_to_water: HashMap::new(),
        water_to_light: HashMap::new(),
        light_to_temperature: HashMap::new(),
        temperature_to_humidity: HashMap::new(),
        humidity_to_location: HashMap::new()};

    let mut current_command: &str = "";
    for line in input.lines() {
       if line.starts_with("seeds: ") {
            mapper.read_seeds(line.strip_prefix("seeds: ").unwrap());
       } else if line == "" {
            current_command = "";
       } else if line.chars().nth(0).unwrap().is_alphabetic() {
        current_command = line;
       }
       else {
        mapper.read_map(current_command, line);
       }
    }
    let mut i = 0; 
    let mut even_seed = 0;
    for seed in mapper.get_seeds().into_iter() { 
        if i % 2 == 0 {
            even_seed = seed;
            let soil = mapper.get_destination(seed, &mapper.seed_to_soil);
            let fertilizer = mapper.get_destination(soil, &mapper.soil_to_fertilizer);
            let water = mapper.get_destination(fertilizer, &mapper.fertilizer_to_water);
            let light = mapper.get_destination(water, &mapper.water_to_light);
            let temperature = mapper.get_destination(light, &mapper.light_to_temperature);
            let humidity = mapper.get_destination(temperature, &mapper.temperature_to_humidity);
            let location = mapper.get_destination(humidity, &mapper.humidity_to_location);
            if location < minimum {
                minimum = location;
            }
            i = 1;
        } else {
            let mut counter = seed;
            while counter != 0 {
                let soil = mapper.get_destination(even_seed, &mapper.seed_to_soil);
                let fertilizer = mapper.get_destination(soil, &mapper.soil_to_fertilizer);
                let water = mapper.get_destination(fertilizer, &mapper.fertilizer_to_water);
                let light = mapper.get_destination(water, &mapper.water_to_light);
                let temperature = mapper.get_destination(light, &mapper.light_to_temperature);
                let humidity = mapper.get_destination(temperature, &mapper.temperature_to_humidity);
                let location = mapper.get_destination(humidity, &mapper.humidity_to_location);
                if location < minimum {
                    minimum = location;
                }
                even_seed += 1;
                counter -= 1;
            }
            i = 0;
        }
        
    }
    return minimum;
}

fn main() {
    let  sum = solve("./src/bin/part1_example.txt");
    println!("{}", sum);
}

struct Mapper {
    seeds: Vec<i64>,
    seed_to_soil: HashMap<i64, (i64, i64)>,
    soil_to_fertilizer : HashMap<i64, (i64, i64)>,
    fertilizer_to_water: HashMap<i64, (i64, i64)>,
    water_to_light: HashMap<i64, (i64, i64)>,
    light_to_temperature: HashMap<i64, (i64, i64)>,
    temperature_to_humidity: HashMap<i64, (i64, i64)>,
    humidity_to_location: HashMap<i64, (i64, i64)>,
}

impl Mapper {
    pub fn read_seeds(&mut self, line: &str) {
        self.seeds = line.split(' ').collect::<Vec<&str>>().iter().map(|x|->i64{x.parse().unwrap()}).collect();
    }

    pub fn read_map(&mut self, map: &str, line: &str) {
        let numbers: Vec<_> = line.split(' ').collect::<Vec<&str>>().iter().map(|x|->i64{x.parse().unwrap()}).collect();
        match map {
            "seed-to-soil map:" => self.seed_to_soil.insert(numbers[1], (numbers[0], numbers[2])),
            "soil-to-fertilizer map:" => self.soil_to_fertilizer.insert(numbers[1], (numbers[0], numbers[2])),
            "fertilizer-to-water map:" => self.fertilizer_to_water.insert(numbers[1], (numbers[0], numbers[2])),
            "water-to-light map:" => self.water_to_light.insert(numbers[1], (numbers[0], numbers[2])),
            "light-to-temperature map:" => self.light_to_temperature.insert(numbers[1], (numbers[0], numbers[2])),
            "temperature-to-humidity map:" => self.temperature_to_humidity.insert(numbers[1], (numbers[0], numbers[2])),
            "humidity-to-location map:" => self.humidity_to_location.insert(numbers[1], (numbers[0], numbers[2])),
            _ => {Some((0,0))},
        };
    }
    pub fn print_maps(&mut self) {
        println!("self.seed_to_soil {:?}", self.seed_to_soil);
        println!("self.soil_to_fertilizer {:?}", self.soil_to_fertilizer);
        println!("self.fertilizer_to_water {:?}", self.fertilizer_to_water);
        println!("self.water_to_light {:?}", self.water_to_light);
        println!("self.light_to_temperature {:?}", self.light_to_temperature);
        println!("self.temperature_to_humidity {:?}", self.temperature_to_humidity);
        println!("self.humidity_to_location {:?}", self.humidity_to_location);
    }
    pub fn get_destination(&self, num:i64, map: &HashMap<i64, (i64, i64)>) -> i64 {
        let mut biggest_key = 0;
        for key in map.clone().into_keys().collect::<Vec<_>>().iter().sort_by(|&&p| p) {
             if *key > num {
                break;
             }
             else {
                biggest_key = *key;
             }
        }
        if map.contains_key(&biggest_key){
            let value = map.get(&biggest_key).unwrap();
            let distance = num - biggest_key;
            if distance <= value.1 {
                return value.0 + distance;
            }
            else {
                return num;
            }
        } else {
            return num;
        }
        
    }

    pub fn get_seeds(&self) -> Vec<i64>{
        return self.seeds.clone();
    }
}
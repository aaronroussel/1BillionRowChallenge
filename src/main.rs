use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

// first run time : 978.85 seconds
// second run : 99.62 seconds -- CPU: Apple M3 Pro 11C

fn main() {
    let start = Instant::now();
    let mut hash_map: DataMap = DataMap {
        hash_map: HashMap::with_capacity(10000),
    };
    let FILENAME = "measurements.txt";
    println!("{}", FILENAME);

    read_and_parse(FILENAME, &mut hash_map);

    for value in hash_map.get_mut() {
        let city = value.0.to_string();
        let temperature_avg: f64 = value.1.get_avg_temp();
        println!("City: {} ---- Average: {}", city, temperature_avg);
    }
    let elapsed = start.elapsed().as_secs_f64();
    println!("Elapsed time: {}", elapsed);
}

fn parse_lines(lines: String) -> std::io::Result<Vec<(String, f64)>> {
    let mut vec = Vec::new();
    for line in lines.lines() {
        let split_string: Vec<&str> = line.trim().split(';').collect();
        let city = split_string[0].to_string();
        let temperature: f64 = split_string[1].parse().unwrap();
        let tuple = (city, temperature);
        vec.push(tuple);
    }
    Ok(vec)
}

fn read_and_parse(filename: &str, map: &mut DataMap) {
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut threads = Vec::new();
    let MAX_THREADS = 2048;
    let NUMBER_OF_LINES = 50000;
    let mut running = true;

    // need a better way to do this concurrently, while avoiding the use of a mutex (because its slow)
    //
    // also may be worth trying to parse the temperature data as an integer instead of a float value,
    // parsing a float probably has more overhead compared to an int, but we sacrifice accuracy due to losing anything after the decimal place.
    // try rounding maybe?
    //
    // Also, reading through line by line may be slow even though we are using a buffered reader.
    // reading in chunks by n amount of bytes might be faster but ensuring we aren't ending at the middle of a line would be complicated.
    // need to look at the implementation of BufReader to get a better understanding of what's going on under the hood.

    while running {
        let (tx, rx) = mpsc::channel();
        for _x in 1..MAX_THREADS {
            if !running {
                break;
            }
            let sender = tx.clone();
            let mut string_buffer = String::new();
            let mut done_reading = false;
            for _v in 1..NUMBER_OF_LINES {
                let read_bytes = buf_reader.read_line(&mut string_buffer).unwrap();
                if read_bytes == 0 {
                    done_reading = true;
                    break;
                }
            }
            let child = thread::spawn(move || {
                let parsed = parse_lines(string_buffer.to_string()).unwrap();
                let _ = sender.send(parsed);
            });
            threads.push(child);
            if done_reading {
                running = false;
                break;
            }
        }
        let dataset = rx.recv().unwrap();
        for data in dataset {
            map.add_data_to_map(data);
        }
    }
}

// paying mind to how we are allocating data may be worth looking into.

struct WeatherData {
    _city: String,
    sum: f64,
    count: i64,
    min: f64,
    max: f64,
}

impl WeatherData {
    pub fn add_data(&mut self, data: f64) {
        self.sum = self.sum + data;
        if data > self.max {
            self.max = data;
        }
        if data < self.min {
            self.min = data;
        }
        self.count = self.count + 1;
    }
    pub fn get_avg_temp(&self) -> f64 {
        let avg = self.sum / self.count as f64;
        avg
    }
}

struct DataMap {
    hash_map: HashMap<String, WeatherData>,
}

impl DataMap {
    fn add_data_to_map(&mut self, data: (String, f64)) {
        if self.hash_map.contains_key(&data.0) {
            let weather_data_object = self
                .hash_map
                .get_mut(&data.0)
                .expect("Error fetching object from map");
            weather_data_object.add_data(data.1);
        } else {
            let new_weather_data_object = WeatherData {
                _city: data.0.to_string(),
                sum: data.1,
                count: 1,
                min: data.1,
                max: data.1,
            };
            let city = data.0.to_string();
            self.hash_map.insert(city, new_weather_data_object);
        }
    }

    fn get_mut(&mut self) -> &mut HashMap<String, WeatherData> {
        let map = &mut self.hash_map;
        map
    }
}

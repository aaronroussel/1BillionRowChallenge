use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

use std::time::Instant;

// first run time : 978.85 seconds

fn main() {
    let start = Instant::now();
    let mut hash_map: HashMap<String, WeatherData> = HashMap::with_capacity(10000);
    let filename = "foo.txt";
    println!("{}", filename);

    let parsed_data = read_lines(filename).unwrap();
    for data in parsed_data {
        let city = data.0;
        let temperature = data.1;
        if hash_map.contains_key(&city) {
            let weather_data_object: &mut WeatherData = hash_map.get_mut(&city).unwrap();
            weather_data_object.add_data(temperature);
        } else {
            let weather_data_object = WeatherData {
               _city: city.to_string(),
                sum: temperature,
                count: 1,
                min: temperature,
                max: temperature,
            };
            hash_map.insert(city, weather_data_object);
        }
    }
    for value in hash_map {
        let city = value.0.to_string();
        let temperature_avg = value.1.get_avg_temp();
        println!("City: {} ---- Average: {}", city, temperature_avg);
    }
    let elapsed = start.elapsed().as_secs_f64();
    println!("Elapsed time: {}", elapsed);
}

fn read_lines(filename: &str) -> std::io::Result<Vec<(String, f64)>> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut parsed_data = Vec::new();
    let mut s: String = String::new();
    let mut count: i64 = 0;
    loop {
        let x = buf_reader.read_line(&mut s).unwrap();
        if x == 0 {
            break;
        }
        count = count + 1;

        let split_string: Vec<&str> = s.trim().split(';').collect();
        let city = split_string[0].to_string();
        let temperature: f64 = split_string[1]
            .parse()
            .expect(&s);
        let data = (city, temperature);
        parsed_data.push(data);
        s.clear();
    }
    Ok(parsed_data)
}

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
    pub fn get_avg_temp(self) -> f64 {
        let avg = self.sum / self.count as f64;
        avg
    }
}

use std::collections::HashMap;
use std::env;
use std::io::{prelude::*, Result};
use std::{fs::File, io::BufReader};

struct MeasurementCounter {
    min: i32,
    max: i32,
    sum: i64,
    count: i32,
}

fn read_line(buffer: String) -> (String, i32) {
    let (city, temp_str) = buffer.trim().split_once(";").unwrap();

    let temp = temp_str.parse::<f32>().unwrap().ceil() as i32;

    (city.to_string(), temp)
}

fn update_map(
    mut map: HashMap<String, MeasurementCounter>,
    city: String,
    temp: i32,
) -> HashMap<String, MeasurementCounter> {
    map.entry(city)
        .and_modify(|measurement| {
            measurement.sum += i64::from(temp);
            measurement.count += 1;

            if measurement.min > temp {
                measurement.min = temp;
            };

            if measurement.max < temp {
                measurement.max = temp;
            };
        })
        .or_insert(MeasurementCounter {
            min: temp,
            max: temp,
            sum: i64::from(temp),
            count: 1,
        });

    map
}

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let file_path = &args[1];
    let mut measurement_counts = HashMap::<String, MeasurementCounter>::new();

    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);

    let mut buffer = String::new();

    while buf_reader.read_line(&mut buffer)? > 0 {
        let (city, temp) = read_line(buffer.clone());
        measurement_counts = update_map(measurement_counts, city, temp);
        buffer.clear();
    }

    measurement_counts.iter().for_each(|(key, measurement)| {
        let min = measurement.min;
        let max = measurement.max;
        let sum = measurement.sum;
        let count = measurement.count;
        let average = sum / i64::from(count);

        println!("{key};{min};{max};{average}")
    });

    Ok(())
}

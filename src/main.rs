use std::collections::HashMap;
use std::env;
use std::io::{prelude::*, Result as IoResult};
use std::{fs::File, io::BufReader};

struct MeasurementCounter {
    min: i32,
    max: i32,
    sum: i64,
    count: i32,
}

enum ReadLineError {
    SplitFailed,
    TempParseFailed,
}

fn read_line(buffer: String) -> Result<(String, i32), ReadLineError> {
    let (city, temp_str) = buffer
        .trim()
        .split_once(";")
        .ok_or(ReadLineError::SplitFailed)?;

    let temp = temp_str
        .parse::<f32>()
        .map_err(|_| ReadLineError::TempParseFailed)?
        .ceil() as i32;

    Ok((city.to_string(), temp))
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

fn main() -> IoResult<()> {
    let args = env::args().collect::<Vec<_>>();
    let file_path = &args[1];
    let mut measurement_counts = HashMap::<String, MeasurementCounter>::new();

    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);

    let mut buffer = [0u8; 512];
    let mut tail = String::new();

    while buf_reader.read(&mut buffer)? > 0 {
        let mut content = std::str::from_utf8(&buffer).unwrap();

        let temp_content = tail.clone() + content;
        content = &temp_content;

        if !content.ends_with("\n") {
            let last_line = content.rsplit("\n").next().unwrap();
            tail = last_line.to_string();
            content = &content[..content.len() - last_line.len()];
        } else {
            tail.clear();
        }

        for line in content.split("\n") {
            if !line.is_empty() {
                let (city, temp) = read_line(line.into()).unwrap_or_else(|_| {
                    eprintln!("Failed to parse line: {}", line);
                    panic!();
                });
                measurement_counts = update_map(measurement_counts, city, temp);
            }
        }
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

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

fn update_map(map: &mut HashMap<String, MeasurementCounter>, city: String, temp: i32) {
    let entry = map.entry(city).or_insert(MeasurementCounter {
        min: temp,
        max: temp,
        sum: i64::from(temp),
        count: 1,
    });

    entry.sum += i64::from(temp);
    entry.count += 1;

    if entry.min > temp {
        entry.min = temp;
    };

    if entry.max < temp {
        entry.max = temp;
    };
}

fn main() -> IoResult<()> {
    let args = env::args().collect::<Vec<_>>();
    let file_path = &args[1];
    let mut measurement_counts = HashMap::<String, MeasurementCounter>::new();

    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);

    let mut buffer = [0u8; 512];
    let mut tail = String::new();
    let mut partial_chars = Vec::<u8>::new();

    while buf_reader.read(&mut buffer)? > 0 {
        let mut new_buf = partial_chars.clone();
        new_buf.extend_from_slice(&buffer);
        let new_buf_ref = new_buf.as_ref();
        let mut content: String = match std::str::from_utf8(new_buf_ref) {
            Ok(content) => {
                partial_chars.clear();
                content.to_string()
            }
            Err(err) => {
                let content =
                    std::str::from_utf8(new_buf_ref[..err.valid_up_to()].as_ref()).unwrap();
                partial_chars = new_buf[err.valid_up_to()..].to_vec();
                content.to_string()
            }
        };

        content.insert_str(0, tail.as_str());

        if !content.ends_with("\n") {
            let last_line = content.rsplit("\n").next().unwrap();
            tail = last_line.to_string();
            content = content[..content.len() - last_line.len()].to_string();
        } else {
            tail.clear();
        }

        for line in content.split("\n") {
            if !line.is_empty() {
                let (city, temp) = read_line(line.into()).unwrap_or_else(|_| {
                    eprintln!("Failed to parse line: {}", line);
                    panic!();
                });
                update_map(&mut measurement_counts, city, temp);
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

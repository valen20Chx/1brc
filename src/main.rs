use std::collections::HashMap;
use std::io::prelude::*;
use std::{env, io};
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

fn read_line(buffer: &str) -> Result<(String, i32), ReadLineError> {
    let (city, temp_str) = buffer
        .trim()
        .split_once(";")
        .ok_or(ReadLineError::SplitFailed)?;

    let temp = temp_str
        .parse::<f32>()
        .map_err(|_| ReadLineError::TempParseFailed)?
        .ceil() as i32;

    Ok((city.to_owned(), temp))
}

fn update_map(map: &mut HashMap<String, MeasurementCounter>, city: String, temp: i32) {
    let entry = map.entry(city).or_insert(MeasurementCounter {
        min: temp,
        max: temp,
        sum: temp as i64,
        count: 1,
    });

    entry.sum += temp as i64;
    entry.count += 1;
    entry.min = entry.min.min(temp);
    entry.max = entry.max.max(temp);
}

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let mut measurement_counts = HashMap::<String, MeasurementCounter>::new();

    let file = File::open(file_path).unwrap_or_else(|_| {
        eprintln!("Failed to open file: {}", file_path);
        std::process::exit(1);
    });

    let buf_reader = BufReader::new(file);

    for line_res in buf_reader.lines() {
        let line = line_res?;
        if let Ok((city, temp)) = read_line(&line) {
            update_map(&mut measurement_counts, city, temp);
        } else {
            eprintln!("Failed to parse line: {}", line);
        }
    }

    for (city, data) in measurement_counts {
        let average = data.sum / i64::from(data.count);

        println!(
            "{key};{min};{max};{average}",
            key = city,
            min = data.min,
            max = data.max,
            average = average
        );
    }

    Ok(())
}

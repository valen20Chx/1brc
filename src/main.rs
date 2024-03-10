use std::collections::HashMap;
use std::io::prelude::*;
use std::sync::mpsc;
use std::{env, io, thread};
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

    if args.len() != 2 && args.len() != 3 {
        eprintln!("Usage: {} <file> <?thread_count>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let file = File::open(file_path).unwrap_or_else(|_| {
        eprintln!("Failed to open file: {}", file_path);
        std::process::exit(1);
    });

    let file_size = file.metadata()?.len();
    let num_threads: u64 = args.get(2).map_or("4", |s| s.as_str()).parse().unwrap_or(4);
    let chunk_size = file_size / num_threads as u64;

    let (tx, rx) = mpsc::channel::<HashMap<String, MeasurementCounter>>();

    let mut threads = vec![];

    for thread_index in 0..num_threads {
        let file_path = file_path.to_owned();

        let tx = tx.clone();

        let thread = thread::spawn(move || -> io::Result<()> {
            let mut file = File::open(&file_path)?;

            let start = if thread_index > 0 {
                let start_temp = thread_index * chunk_size;
                file.seek(io::SeekFrom::Start(start_temp - 1))?;

                let mut buf_reader = BufReader::new(&file);
                let mut buffer = vec![];

                buf_reader.read_until(b'\n', &mut buffer)?;

                start_temp + buffer.len() as u64 - 1
            } else {
                thread_index * chunk_size
            };

            let end = if thread_index < num_threads - 1 {
                let temp_end = start + chunk_size;
                file.seek(io::SeekFrom::Start(temp_end))?;

                let mut buf_reader = BufReader::new(&file);
                let mut buffer = vec![];

                buf_reader.read_until(b'\n', &mut buffer)?;

                temp_end + buffer.len() as u64 - 1
            } else {
                file_size
            };

            let mut file = File::open(file_path)?;
            file.seek(io::SeekFrom::Start(start))?;

            let file = file.take(end - start);

            let buf_reader = BufReader::new(file);

            let mut measurement_counts = HashMap::<String, MeasurementCounter>::new();

            for line in buf_reader.lines() {
                let line = line?;
                if let Ok((city, temp)) = read_line(&line) {
                    update_map(&mut measurement_counts, city, temp);
                } else {
                    eprintln!("Failed to parse line: {}", line);
                }
            }

            tx.send(measurement_counts).unwrap();
            Ok(())
        });

        threads.push(thread);
    }

    drop(tx);

    let mut measurement_counts = HashMap::<String, MeasurementCounter>::new();

    for _ in 0..num_threads {
        let thread_measurements = rx.recv().unwrap();

        for (city, data) in thread_measurements {
            let entry = measurement_counts
                .entry(city)
                .or_insert(MeasurementCounter {
                    min: data.min,
                    max: data.max,
                    sum: data.sum,
                    count: data.count,
                });
            entry.sum += data.sum;
            entry.count += data.count;
            entry.min = entry.min.min(data.min);
            entry.max = entry.max.max(data.max);
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

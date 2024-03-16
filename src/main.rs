use std::collections::HashMap;
use std::io::prelude::*;
use std::sync::mpsc;
use std::{env, io, thread};
use std::{fs::File, io::BufReader};

#[derive(Clone)]
struct MeasurementCounter {
    min: i32,
    max: i32,
    sum: i64,
    count: i32,
}

impl std::fmt::Display for MeasurementCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{min={}, max={}, sum={}, count={}}}",
            self.min, self.max, self.sum, self.count
        )
    }
}

enum ReadLineError {
    SplitFailed,
    TempParseFailed,
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
            let measurements =
                process_file_chunk(file_path, thread_index, chunk_size, num_threads, file_size)?;

            tx.send(measurements).unwrap();
            Ok(())
        });

        threads.push(thread);
    }

    drop(tx);

    let measurement_counts = merge_threads_measurements(num_threads, rx);

    for (city, data) in measurement_counts {
        let average = data.sum / i64::from(data.count);

        println!(
            "{key};{min};{max};{average}",
            key = city,
            min = data.min,
            max = data.max,
            average = average,
        );
    }

    Ok(())
}

fn merge_threads_measurements(
    num_threads: u64,
    rx: mpsc::Receiver<HashMap<String, MeasurementCounter>>,
) -> HashMap<String, MeasurementCounter> {
    let mut measurement_counts = HashMap::<String, MeasurementCounter>::new();
    for _ in 0..num_threads {
        let thread_measurements = rx.recv().unwrap();

        for (city, data) in thread_measurements {
            let entry = measurement_counts
                .entry(city.clone())
                .or_insert(MeasurementCounter {
                    min: i32::MAX,
                    max: i32::MIN,
                    sum: 0,
                    count: 0,
                });

            entry.sum += data.sum;
            entry.count += data.count;
            entry.min = entry.min.min(data.min);
            entry.max = entry.max.max(data.max);
        }
    }
    measurement_counts
}

fn process_file_chunk(
    file_path: String,
    chunk_index: u64,
    chunk_size: u64,
    num_chunks: u64,
    file_size: u64,
) -> Result<HashMap<String, MeasurementCounter>, io::Error> {
    let limited_reader =
        get_chunk_file_reader(file_path, chunk_index, chunk_size, num_chunks, file_size)?;

    let mut measurement_counts = HashMap::<String, MeasurementCounter>::new();

    for line in limited_reader.lines() {
        let line = line?;
        if let Ok((city, temp)) = read_line(&line) {
            update_map(&mut measurement_counts, city, temp);
        } else {
            eprintln!("Failed to parse line: {}", line);
        }
    }

    Ok(measurement_counts)
}

fn get_chunk_file_reader(
    file_path: String,
    chunk_index: u64,
    chunk_size: u64,
    num_chunks: u64,
    file_size: u64,
) -> Result<io::Take<BufReader<File>>, io::Error> {
    let file = File::open(&file_path)?;
    let mut buf_reader = BufReader::new(file);

    let start = if chunk_index > 0 {
        let start_temp = chunk_index * chunk_size;
        buf_reader.seek(io::SeekFrom::Start(start_temp))?;

        let mut buffer = vec![];
        buf_reader.read_until(b'\n', &mut buffer)?;

        start_temp + buffer.len() as u64
    } else {
        0
    };

    let end = if chunk_index < num_chunks - 1 {
        let temp_end = (chunk_index + 1) * chunk_size;
        buf_reader.seek(io::SeekFrom::Start(temp_end))?;

        let mut buffer = vec![];
        buf_reader.read_until(b'\n', &mut buffer)?;

        temp_end + buffer.len() as u64
    } else {
        file_size
    };

    buf_reader.seek(io::SeekFrom::Start(start))?;

    Ok(buf_reader.take(end - start))
}

fn update_map(map: &mut HashMap<String, MeasurementCounter>, city: String, temp: i32) {
    let entry = map.entry(city).or_insert(MeasurementCounter {
        min: i32::MAX,
        max: i32::MIN,
        sum: 0,
        count: 0,
    });

    entry.sum += temp as i64;
    entry.count += 1;
    entry.min = entry.min.min(temp);
    entry.max = entry.max.max(temp);
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

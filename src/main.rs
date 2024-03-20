mod file;
mod measurements;

use std::collections::HashMap;
use std::fs::File;
use std::sync::mpsc;
use std::{env, io, thread};

use file::process_file_chunk;
use measurements::{merge_threads_measurements, MeasurementCounter};

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

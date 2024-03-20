use std::collections::HashMap;
use std::io::prelude::*;
use std::{
    fs::File,
    io::{self, BufReader},
};

use crate::measurements::{update_map, MeasurementCounter};

pub enum ReadLineError {
    SplitFailed,
    TempParseFailed,
}

pub fn read_line(buffer: &str) -> Result<(String, i32), ReadLineError> {
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

pub fn get_chunk_file_reader(
    file_path: String,
    chunk_index: u64,
    chunk_size: u64,
    num_chunks: u64,
    file_size: u64,
) -> Result<io::Take<BufReader<File>>, io::Error> {
    let file = File::open(&file_path)?;
    let mut buf_reader = BufReader::with_capacity(8192 * 1, file);

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

pub fn process_file_chunk(
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

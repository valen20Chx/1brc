use std::{collections::HashMap, sync::mpsc};

#[derive(Clone)]
pub struct MeasurementCounter {
    pub min: i32,
    pub max: i32,
    pub sum: i64,
    pub count: i32,
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

pub fn merge_threads_measurements(
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

pub fn update_map(map: &mut HashMap<String, MeasurementCounter>, city: String, temp: i32) {
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

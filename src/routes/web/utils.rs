use chrono::{DateTime, Duration, Timelike, Utc};
use std::{cmp::min, collections::HashMap};

use super::types::*;

pub fn aggregate_flowmeter(entries: Vec<FlowmeterPoint>) -> Vec<IrrigationPoint> {
    let mut hourly_map: HashMap<DateTime<Utc>, f64> = HashMap::new();

    for entry in entries {
        if entry.stop <= entry.start {
            continue;
        }

        let total_duration_secs = (entry.stop - entry.start).num_seconds() as f64;
        let mut current = entry.start;

        while current < entry.stop {
            let bucket = truncate_to_hour(current);
            let next_hour = bucket + Duration::hours(1);
            let segment_end = min(next_hour, entry.stop);
            let duration = (segment_end - current).num_seconds() as f64;
            let fraction = duration / total_duration_secs;

            *hourly_map.entry(bucket).or_insert(0.0) += entry.value * fraction;
            current = segment_end;
        }
    }

    let mut result: Vec<IrrigationPoint> = hourly_map
        .into_iter()
        .map(|(time, value)| IrrigationPoint { time, value })
        .collect();

    result.sort_by_key(|entry| entry.time);
    result
}

fn truncate_to_hour(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.with_minute(0)
        .and_then(|dt| dt.with_second(0))
        .and_then(|dt| dt.with_nanosecond(0))
        .unwrap()
}

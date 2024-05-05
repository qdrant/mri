use crate::args::MergeOutput;
use crate::common::{DELAY, TIMESTAMP};
use crate::save_html::{convert_data_to_html, render_html};
use std::collections::HashMap;
use std::io::BufRead;

fn read_jsonl_file(path: &str) -> Vec<HashMap<String, f64>> {
    let file_res = std::fs::File::open(path);

    // Report file open error nicely
    let file = match file_res {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file `{}`: {}", path, e);
            std::process::exit(1);
        }
    };

    let reader = std::io::BufReader::new(file);
    let mut data = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let record: HashMap<String, f64> = serde_json::from_str(&line).unwrap();
        data.push(record);
    }
    data
}

fn get_time_key(data: &HashMap<String, f64>) -> &'static str {
    if data.contains_key(DELAY) {
        DELAY
    } else if data.contains_key(TIMESTAMP) {
        TIMESTAMP
    } else {
        eprintln!("Error: Record does not contain key `delay` or `timestamp`");
        std::process::exit(1);
    }
}

/// Read value of either `delay` or `timestamp` key from record
fn get_time_related_value(record: &HashMap<String, f64>) -> f64 {
    let key = get_time_key(record);
    *record.get(key).unwrap()
}

fn convert_timestamp_to_relative(data: &mut Vec<HashMap<String, f64>>) {
    let first_record = data.first().unwrap();

    let time_key = get_time_key(first_record);
    if time_key == DELAY {
        return;
    }

    let first_value = *first_record.get(TIMESTAMP).unwrap();

    for record in data {
        let value = record.remove(TIMESTAMP).unwrap();
        record.insert(DELAY.to_string(), value - first_value);
    }
}

fn set_time_offset(data: &mut Vec<HashMap<String, f64>>, offset: f64) {
    for record in data {
        let key = get_time_key(record);
        let value = record.get_mut(key).unwrap();
        *value += offset;
    }
}


/// Given the maximum amount of data points to keep, thin out the data
/// by selecting at most the given amount of data points.
/// The data points are selected evenly from the input data.
fn thin_out_data<T>(data: Vec<T>, sample: usize) -> Vec<T> {
    if data.len() <= sample {
        return data;
    }

    // Always less than 1 and greater than 0
    let target_ratio = sample as f64 / data.len() as f64;
    let mut result = vec![];

    for (idx, record) in data.into_iter().enumerate() {
        if result.len() == sample {
            break;
        }
        let running_ratio = result.len() as f64 / (idx + 1) as f64;
        if running_ratio < target_ratio {
            result.push(record);
        }
    }
    result
}

fn preprocess_data(
    mut data: Vec<HashMap<String, f64>>,
    cut_to: Option<f64>,
    cut_from: Option<f64>,
    offset: Option<f64>,
    convert_to_relative: bool,
    sample: Option<usize>,
) -> Vec<HashMap<String, f64>> {
    data.sort_unstable_by(|a, b| {
        let a_time = get_time_related_value(a);
        let b_time = get_time_related_value(b);
        a_time.partial_cmp(&b_time).unwrap()
    });

    if let Some(sample) = sample {
        data = thin_out_data(data, sample);
    }

    if convert_to_relative {
        convert_timestamp_to_relative(&mut data);
    }

    if let Some(offset) = offset {
        set_time_offset(&mut data, offset);
    }

    data.into_iter()
        .filter(|record| {
            let time = get_time_related_value(record);

            if let Some(cut_to) = cut_to {
                if time > cut_to {
                    return false;
                }
            }

            if let Some(cut_from) = cut_from {
                if time < cut_from {
                    return false;
                }
            }

            true
        })
        .collect()
}

pub fn merge_results(args: MergeOutput) {
    let mut output_data = vec![];

    for (idx, file) in args.jsonl.iter().enumerate() {
        let data = read_jsonl_file(file);
        let offset = args.offset.get(idx).cloned();
        let processed_data = convert_data_to_html(&preprocess_data(
            data,
            args.cut_to,
            args.cut_from,
            offset,
            args.to_relative.unwrap_or_default(),
            args.sample,
        ));
        output_data.push(processed_data);
    }

    let json_values: Vec<_> = output_data
        .into_iter()
        .zip(args.jsonl.iter())
        .map(|(data, file)| {
            let mut value = serde_json::to_value(data).unwrap();
            value
                .as_object_mut()
                .unwrap()
                .insert("name".to_string(), serde_json::Value::String(file.clone()));
            value
        })
        .collect();

    render_html(&args.html, &json_values).unwrap();
}

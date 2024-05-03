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

fn set_time_offset(data: &mut Vec<HashMap<String, f64>>, offset: f64) {
    for record in data {
        let key = get_time_key(record);
        let value = record.get_mut(key).unwrap();
        *value += offset;
    }
}

fn preprocess_data(
    mut data: Vec<HashMap<String, f64>>,
    cut_to: Option<f64>,
    cut_from: Option<f64>,
    offset: Option<f64>,
) -> Vec<HashMap<String, f64>> {
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
        let processed_data =
            convert_data_to_html(&preprocess_data(data, args.cut_to, args.cut_from, offset));
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

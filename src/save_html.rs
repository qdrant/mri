use std::collections::HashMap;
use std::io::Write;
use serde::Serialize;

static HTML_FILE: &str = include_str!("../data/plot.html");


pub fn convert_data_to_html(data: &[HashMap<String, f64>]) -> HashMap<String, Vec<f64>> {
    let mut converted_data: HashMap<String, Vec<f64>> = HashMap::new();

    for record in data {
        for (key, value) in record {
            let entry = converted_data.entry(key.clone()).or_default();
            entry.push(*value);
        }
    }

    converted_data
}

pub fn render_html<T: ?Sized + Serialize>(path: &str, data: &T) -> std::io::Result<()>
{
    let json_data = serde_json::to_string(&data).unwrap();

    let html_file = HTML_FILE.replace("%DATA%", &json_data);

    let mut file = std::fs::File::create(path)?;
    writeln!(&mut file, "{}", html_file).unwrap();
    Ok(())
}

pub fn save_data_as_html(path: &str, data: &[HashMap<String, f64>]) -> std::io::Result<()> {
    let converted_data = convert_data_to_html(data);

    render_html(path, &converted_data)
}


use std::collections::HashMap;
use std::io::Write;

static HTML_FILE: &'static str = include_str!("../data/plot.html");


pub fn save_data_as_html(path: &str, _data: &[HashMap<String, f64>]) -> std::io::Result<()> {
    let mut converted_data: HashMap<String, Vec<f64>> = HashMap::new();


    for record in _data {
        for (key, value) in record {
            let entry = converted_data.entry(key.clone()).or_insert(vec![]);
            entry.push(*value);
        }
    }

    let json_data = serde_json::to_string(&converted_data).unwrap();

    let html_file = HTML_FILE.replace("%DATA%", &json_data);

    let mut file = std::fs::File::create(path)?;
    writeln!(&mut file, "{}", html_file).unwrap();
    Ok(())
}

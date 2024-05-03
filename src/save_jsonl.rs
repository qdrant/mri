use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub fn save_data_as_jsonl(path: &str, data: &[HashMap<String, f64>]) -> std::io::Result<()> {
    if path == "-" {
        for record in data {
            let json = serde_json::to_string(&record)?;
            println!("{}", json);
        }
        return Ok(());
    }

    let mut file = File::options().append(true).create(true).open(path)?;

    for record in data {
        let json = serde_json::to_string(&record)?;
        writeln!(&mut file, "{}", json)?;
    }

    Ok(())
}

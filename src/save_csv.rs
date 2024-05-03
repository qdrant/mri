use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub fn save_csv(path: &str, data: &[HashMap<String, f64>]) -> std::io::Result<()> {
    if data.is_empty() {
        return Ok(());
    }

    let header_columns: Vec<_> = data[0].keys().map(String::as_str).collect();

    if path == "-" {
        // Print header
        println!("{}", header_columns.join(","));

        for record in data {
            let row: Vec<_> = header_columns
                .iter()
                .map(|key| record.get(*key).unwrap().to_string())
                .collect();
            println!("{}", row.join(","));
        }
        return Ok(());
    }

    let mut file = File::options().write(true).truncate(true).open(path)?;

    // Write header
    writeln!(&mut file, "{}", header_columns.join(",")).unwrap();

    for record in data {
        let row: Vec<_> = header_columns
            .iter()
            .map(|key| record.get(*key).unwrap().to_string())
            .collect();
        writeln!(&mut file, "{}", row.join(",")).unwrap();
    }

    Ok(())
}

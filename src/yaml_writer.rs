use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use crate::model::{DicomEntry, DicomField};
use serde_yaml;

pub fn write_yml(fields: Vec<DicomField>) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();

    for field in fields {
        map.insert(
            field.name,
            DicomEntry {
                tag: field.tag,
                value: field.value,
            },
        );
    }

    let yaml = serde_yaml::to_string(&map)?;
    // create output file as data.yml
    let output_path = format!("data.yml");
    let mut file = File::create(&output_path);
    file?.write_all(yaml.as_bytes())?;

    println!("YAML written to {}", output_path);
    Ok(())
}

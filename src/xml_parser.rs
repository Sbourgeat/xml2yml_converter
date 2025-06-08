use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;

use crate::model::{tag_lookup, DicomField};

pub fn analyze_xml(path: &str) -> Vec<DicomField> {
    let mut buf = Vec::new();
    let mut results = Vec::new();
    let mut current_tag = None;

    let content = fs::read_to_string(path).expect("Should have been able to read the file");

    let mut reader = Reader::from_str(&content);
    reader.config_mut().trim_text(true);

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                current_tag = Some(tag_name);
            }
            Ok(Event::Text(e)) => {
                if let Some(ref tag_name) = current_tag {
                    let value = e.unescape().unwrap().to_string();

                    let tag_map = tag_lookup();
                    let tag_dicom = tag_map.get(tag_name.as_str()).unwrap_or(&"(Unknown)");

                    let field = DicomField {
                        name: tag_name.clone(),
                        tag: tag_dicom.to_string(),
                        value,
                    };

                    results.push(field);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error: {:?}", e),
            _ => {}
        }
    }
    buf.clear();
    results
}

use std::collections::HashMap;

#[derive(Debug, serde_derive::Serialize)]
pub struct DicomField {
    pub name: String,
    pub tag: String,
    pub value: String,
}

pub fn tag_lookup() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("PatientName", "(0010,0010)");
    map.insert("PatientID", "(0010,0020)");
    map.insert("PatientBirthDate", "(0010,0030)");
    map.insert("PatientSex", "(0010,0040)");
    map.insert("StudyDate", "(0008,0020)");
    map
}

#[derive(Debug, serde_derive::Serialize)]
pub struct DicomEntry {
    pub tag: String,
    pub value: String,
}

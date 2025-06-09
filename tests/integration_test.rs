use std::fs::write;
use tempfile::tempdir;

use xml2yml_converter::xml_parser::analyze_xml;

#[test]
fn test_analyze_xml_returns_expected_fields() {
    let mock_xml = r#"
        <DicomDataset>
            <PatientName>John Burrow</PatientName>
            <PatientId>123456</PatientId>
        </DicomDataset>
    "#;

    let dir = tempdir().expect("Cannot create temporary directory");
    let file_path = dir.path().join("test_input.xml");

    write(&file_path, mock_xml).expect("Cannot write the mock XML file");

    let fields = analyze_xml(file_path.to_str().unwrap());
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "PatientName");
    assert_eq!(fields[0].value, "John Burrow");
    assert_eq!(fields[1].name, "PatientId");
    assert_eq!(fields[1].value, "123456");
}

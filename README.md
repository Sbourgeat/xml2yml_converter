# xml2yml_converter
A Rust CLI tool to convert DICOM-style XML metadata into human-readable YAML format.

🚀 Features
Parses DICOM XML using quick-xml
Outputs structured YAML using serde_yaml
Modular and extensible Rust codebase
CLI-friendly — easy to integrate in pipelines

🧪 Example
Input XML:
```XML
<DicomDataset>
<PatientName>DOE^JOHN</PatientName>
<StudyDate>20230824</StudyDate>
</DicomDataset>
```
Output YAML:
```yml
PatientName:
    tag: (0010,0010)
    value: DOE^JOHN
StudyDate:
    tag: (0008,0020)
    value: 20230824
```

🛠 Usage
```bash
cargo run -- path/to/input.xml
```
This will create data.yml in the specified output folder.

📁 Structure
main.rs — CLI handling and orchestration
model.rs — Struct definitions and tag map
xml_parser.rs — XML parsing logic
yaml_writer.rs — YAML serialization logic

📝 License
MIT

🤝 Contributing
Feel free to fork and submit PRs for improvements

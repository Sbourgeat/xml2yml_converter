use std::io;

mod model;
mod xml_parser;
mod yaml_writer;

fn main() {
    println!("Enter the path to the XML file:");
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read input");

    let path = path.trim();
    let res = xml_parser::analyze_xml(path);
    let yml_res = yaml_writer::write_yml(res);
    println!("{:?}", yml_res)
}

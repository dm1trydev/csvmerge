mod result_fields;

use clap::Parser;
use result_fields::ResultFields;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Column number to search in the first file
    #[arg(short, long)]
    source: usize,

    /// Column number to search in the second file
    #[arg(short, long)]
    destination: usize,

    /// List of columns to be taken from the first and second files, respectively. Separated by a comma between themselves and a space between files
    #[arg(short, long, value_parser = ResultFields::parse)]
    result: ResultFields,

    file1_path: std::path::PathBuf,
    file2_path: std::path::PathBuf,

    /// Path to result file
    #[arg(short, long, default_value = "result.csv")]
    output: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut file1 = csv::Reader::from_path(args.file1_path).expect("Must be a CSV file.");
    let mut file2 = csv::Reader::from_path(args.file2_path).expect("Must be a CSV file.");
    let mut content = HashMap::new();

    for record in file2.records() {
        let row = record.expect("a CSV record");

        let union_column = row
            .get(args.destination)
            .expect("a valid UTF-8 string")
            .to_string();

        let result_fields: Vec<String> = args
            .result
            .file2
            .iter()
            .map(|number| row.get(*number).expect("a valid UTF-8 string").to_string())
            .collect();

        content.insert(union_column, result_fields);
    }

    let mut writer = csv::Writer::from_path(args.output).expect("some writer");

    for record in file1.records() {
        let row = record.expect("a CSV record");

        let union_column = row
            .get(args.source)
            .expect("a valid UTF-8 string")
            .to_string();

        let mut result_fields: Vec<String> = args
            .result
            .file1
            .iter()
            .map(|number| row.get(*number).expect("a valid UTF-8 string").to_string())
            .collect();

        let file2_result_fields = match content.get(&union_column) {
            Some(columns) => columns.clone(),
            None => continue,
        };

        result_fields.extend(file2_result_fields);

        writer.write_record(&result_fields).expect("writing error");
    }
}

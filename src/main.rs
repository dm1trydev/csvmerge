mod merge_field;
mod result_fields;

use clap::Parser;
use merge_field::MergeField;
use result_fields::ResultFields;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Формат разделителей первого и второго csv файлов соответственно
    #[arg(short = 'D', long, default_value_t = String::from(",,"))]
    delimeter: String,

    /// Название или номер столбца, для поиска в первом файле
    #[arg(short, long, value_parser = MergeField::parse)]
    source: MergeField, // replace with MergeField

    /// Название или номер столбца, для поиска во втором файле
    #[arg(short, long, value_parser = MergeField::parse)]
    destination: MergeField,

    /// Список полей, которые необходимо взять из первого и второго файлов соответственно. Разделяются запятой между собой и пробелом между файлами
    #[arg(short, long, value_parser = ResultFields::parse)]
    result: ResultFields,

    file1_path: std::path::PathBuf,
    file2_path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    // ID - [0]
    // UUID - [1]
    let mut file1 = csv::Reader::from_path(args.file1_path).expect("Must be a CSV file.");

    // Payment ID - [0]
    // Title - [1]
    let mut file2 = csv::Reader::from_path(args.file2_path).expect("Must be a CSV file.");

    println!("{:?}", args.source);
    println!("{:?}", args.destination);

    let mut content = HashMap::new();

    for record in file2.records() {
        let row = record.expect("a CSV record");

        let union_column = match args.destination {
            MergeField::Number(number) => {
                row.get(number).expect("a valid UTF-8 string").to_string()
            }
            _ => todo!(),
        };

        let result_fields: Vec<String> = args
            .result
            .file2
            .iter()
            .map(|column| match column {
                MergeField::Number(number) => {
                    row.get(*number).expect("a valid UTF-8 string").to_string()
                }
                _ => todo!(),
            })
            .collect();

        content.insert(union_column, result_fields);
    }

    let mut writer = csv::Writer::from_path("result.csv").expect("some writer");

    for record in file1.records() {
        let row = record.expect("a CSV record");

        let union_column = match args.source {
            MergeField::Number(number) => {
                row.get(number).expect("a valid UTF-8 string").to_string()
            }
            _ => todo!(),
        };

        let mut result_fields: Vec<String> = args
            .result
            .file1
            .iter()
            .map(|column| match column {
                MergeField::Number(number) => {
                    row.get(*number).expect("a valid UTF-8 string").to_string()
                }
                _ => todo!(),
            })
            .collect();

        let file2_result_fields = match content.get(&union_column) {
            Some(fields) => fields.clone(),
            None => continue,
        };

        result_fields.extend(file2_result_fields);

        writer.write_record(&result_fields).expect("writing error");
    }
}

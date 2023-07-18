mod merge_field;

use clap::Parser;
use std::collections::HashMap;
use merge_field::MergeField;


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

        let payment_id = match args.source {
            MergeField::Number(number) => row.get(number).expect("a valid UTF-8 string").to_string(),
            _ => todo!(),
        };

        let title = match args.destination {
            MergeField::Number(number) => row.get(number).expect("a valid UTF-8 string").to_string(),
            _ => todo!(),
        };

        content.insert(payment_id, title);
    }

    let mut writer = csv::Writer::from_path("result.csv").expect("some writer");

    for record in file1.records() {
        let row = record.expect("a CSV record");

        let payment_id = row.get(0).expect("a valid UTF-8 string").to_string();
        let uuid = row.get(1).expect("a valid UTF-8 string").to_string();

        let title = match content.get(&payment_id) {
            Some(title) => title.to_string(),
            None => continue,
        };

        writer.write_record(&[uuid, title]).expect("writing error");
    }
}

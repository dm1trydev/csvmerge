use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Формат разделителей первого и второго csv файлов соответственно
    #[arg(short, long, default_value_t = String::from(","))]
    format: String,

    /// Флаг обозначающий наличие или отсутствие заголовков в файлах
    #[arg(short, long, default_value_t = true)]
    no_headers: bool,

    /// название столбца, для поиска соответствий в обоих файлах.
    /// Может принять также позиции столбцов в обоих файлах через `$1`, `$2`, где `$1` - первый столбец первого файла,
    /// `$2` -  второй столбец второго файла.
    #[arg(short, long, default_value_t = String::from("$1"))]
    source: String,

    /// список полей, которые необходимо взять из первого и второго файлов соответственно.
    /// Разделяются запятой между собой и пробелом между файлами
    #[arg(short, long, default_value_t = String::from("$1"))]
    destination: String,

    file1_path: std::path::PathBuf,
    file2_path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    // println!("{:?}", args);

    // ID - [0]
    // UUID - [1]
    let mut file1 = csv::Reader::from_path(args.file1_path).expect("Must be a CSV file.");

    // Payment ID - [0]
    // Title - [1]
    let mut file2 = csv::Reader::from_path(args.file2_path).expect("Must be a CSV file.");

    let mut content = HashMap::new();

    for record in file2.records() {
        let row = record.expect("a CSV record");

        let payment_id = row.get(0).expect("a valid UTF-8 string").to_string();
        let title = row.get(1).expect("a valid UTF-8 string").to_string();

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

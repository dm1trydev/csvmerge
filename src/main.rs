use clap::Parser;

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

    println!("{:?}", args);
}

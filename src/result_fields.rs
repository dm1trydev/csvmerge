#[derive(Debug)]
enum Column {
    Number(usize),
    Name(String),
}

#[derive(Debug)]
pub struct ResultFields {
    file1: Vec<Column>,
    file2: Vec<Column>,
}

impl ResultFields {
    pub fn new(s: &str) -> Self {
        let mut files: Vec<Vec<Column>> = s.split(' ').map(|file| Self::parse_columns(file)).collect();
        let file2 = files.pop().unwrap();
        let file1 = files.pop().unwrap();

        Self { file1, file2 }
    }

    fn parse_columns(s: &str) -> Vec<Column> {
        let mut result: Vec<Column> = Vec::new();

        for column in s.split(',') {
            let number = column.parse::<usize>();

            if number.is_ok() {
                result.push(Column::Number(number.unwrap()));
            } else {
                result.push(Column::Name(column.to_string()));
            }
        }

        result
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        Ok(Self::new(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_source_with_field_number() {
        let result = ResultFields::new("0,1,2 0,2,3");
        let expected_file1 = vec![Column::Number(0), Column::Number(1), Column::Number(2)];
        let expected_file2 = vec![Column::Number(0), Column::Number(2), Column::Number(3)];

        assert_eq!(compare_vecs(&result.file1, &expected_file1), true);
        assert_eq!(compare_vecs(&result.file2, &expected_file2), true);
    }

    #[test]
    fn it_creates_source_with_field_name() {
        let result = ResultFields::new("uuid,title name,score");
        let expected_file1 = vec![Column::Name("uuid".to_string()), Column::Name("title".to_string())];
        let expected_file2 = vec![Column::Name("name".to_string()), Column::Name("score".to_string())];

        assert_eq!(compare_vecs(&result.file1, &expected_file1), true);
        assert_eq!(compare_vecs(&result.file2, &expected_file2), true);
    }

    fn compare_vecs(a: &Vec<Column>, b: &Vec<Column>) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = true;

        for i in 0..a.len() {
            match &a[i] {
                Column::Number(first) => {
                    match &b[i] {
                        Column::Number(second) => {
                            if first != second {
                                result = false;
                            }
                        },
                        _ => { result = false },
                    }
                },
                Column::Name(first) => {
                    match &b[i] {
                        Column::Name(second) => {
                            if first != second {
                                result = false;
                            }
                        },
                        _ => { result = false },
                    }
                },
            }
        }

        result
    }
}

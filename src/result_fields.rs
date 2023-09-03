#[derive(Debug, Clone)]
pub struct ResultFields {
    pub file1: Vec<usize>,
    pub file2: Vec<usize>,
}

impl ResultFields {
    pub fn new(s: &str) -> Self {
        let mut files: Vec<Vec<usize>> = s.split(' ').map(|file| Self::parse_columns(file)).collect();
        let file2 = files.pop().unwrap();
        let file1 = files.pop().unwrap();

        Self { file1, file2 }
    }

    fn parse_columns(s: &str) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        for column in s.split(',') {
            let number = column.parse::<usize>();

            if number.is_ok() {
                result.push(number.unwrap());
            } else {
                panic!("Result fileds must be an array of columns' numbers")
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
        let expected_file1 = vec![0, 1, 2];
        let expected_file2 = vec![0, 2, 3];

        assert_eq!(result.file1, expected_file1);
        assert_eq!(result.file2, expected_file2);
    }
}

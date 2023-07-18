#[derive(Debug, Clone)]
pub enum MergeField {
    Number(usize),
    Name(String),
}

impl MergeField {
    pub fn new(s: &str) -> Self {
        let number = s.parse::<usize>();

        if number.is_ok() {
            MergeField::Number(number.unwrap())
        } else {
            MergeField::Name(s.to_string())
        }
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
        let source_number = 4;
        let result = MergeField::new(&source_number.to_string());

        match result {
            MergeField::Number(number) => assert_eq!(number, source_number),
            _ => panic!("Parsing was incorrect"),
        }
    }

    #[test]
    fn it_creates_source_with_field_name() {
        let source_name = "uuid";
        let result = MergeField::new(source_name);

        match result {
            MergeField::Name(name) => assert_eq!(name, source_name),
            _ => panic!("Parsing was incorrect"),
        }
    }
}

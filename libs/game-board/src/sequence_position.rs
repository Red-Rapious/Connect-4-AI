#[derive(Debug, PartialEq)]
pub struct SequencePosition {
    sequence: Vec<u8>
}

impl From<&String> for SequencePosition {
    fn from(value: &String) -> Self {
        let sequence = value
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        Self {
            sequence
        }
    }
}
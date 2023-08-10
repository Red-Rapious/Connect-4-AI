#[derive(Debug, PartialEq)]
pub struct SequencePosition {
    sequence: Vec<usize>
}

impl SequencePosition {
    pub fn sequence(&self) -> &Vec<usize> {
        &self.sequence
    }
}

impl From<&String> for SequencePosition {
    fn from(value: &String) -> Self {
        let sequence = value
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        Self {
            sequence
        }
    }
}
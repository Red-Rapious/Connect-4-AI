use std::time::Duration;

pub struct Statistics {
    results: Vec<bool>,
    execution_times: Vec<Duration>
}

impl Statistics {
    pub fn new(results: Vec<bool>, execution_times: Vec<Duration>) -> Self {
        assert_eq!(results.len(), execution_times.len());
        assert_ne!(results.len(), 0);

        Self { results, execution_times }
    }

    pub fn mean_time(&self) -> Duration {
        self.execution_times
            .iter()
            .sum::<Duration>() / self.execution_times.len() as u32
    }

    pub fn accuracy(&self) -> f32 {
        let corrects = self.results
            .iter()
            .map(|b| if *b { 1 } else { 0 })
            .sum::<usize>() as f32;

        corrects / self.results.len() as f32
    }

    pub fn results(&self) -> &Vec<bool> {
        &self.results
    }

    pub fn execution_times(&self) -> &Vec<Duration> {
        &self.execution_times
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accuracy() {
        let stats = Statistics::new(
            vec![true, true, true, false], 
            vec![Duration::new(0, 0); 4]);
        
        assert_eq!(stats.accuracy(), 0.75);
    }
}
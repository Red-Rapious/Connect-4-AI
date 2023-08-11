use std::time::Duration;
use std::fmt;

pub struct Statistics {
    results: Vec<bool>,
    execution_times: Vec<Duration>,
    explored_positions_nb: Vec<usize>
}

impl Statistics {
    pub fn new(results: Vec<bool>, execution_times: Vec<Duration>, explored_positions_nb: Vec<usize>) -> Self {
        assert_ne!(results.len(), 0);
        assert_eq!(results.len(), execution_times.len());
        assert_eq!(results.len(), explored_positions_nb.len());

        Self { results, execution_times, explored_positions_nb}
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

    pub fn mean_explored_positions(&self) -> usize {
        self.explored_positions_nb.iter().sum::<usize>() / self.explored_positions_nb.len()
    }

    pub fn results(&self) -> &Vec<bool> {
        &self.results
    }

    pub fn execution_times(&self) -> &Vec<Duration> {
        &self.execution_times
    }

    pub fn explored_positions_nb(&self) -> &Vec<usize> {
        &self.explored_positions_nb
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Accuracy: {}%; Mean time: {:?}; Mean explored positions: {}", 
            (self.accuracy() * 100.0) as usize, 
            self.mean_time(),
            self.mean_explored_positions()
        )
    }
}

#[cfg(test)]
mod statistics_tests {
    use super::*;

    #[test]
    fn accuracy() {
        let stats = Statistics::new(
            vec![true, true, true, false], 
            vec![Duration::new(0, 0); 4],
        vec![0; 4]);
        
        assert_eq!(stats.accuracy(), 0.75);
    }

    #[test]
    fn display() {
        let stats = Statistics::new(
            vec![true, true, true, false], 
            vec![Duration::new(0, 0); 4],
            vec![0; 4]);

        assert_eq!(format!("{}", stats), "Accuracy: 75%; Mean time: 0ns; Mean explored positions: 0".to_string());
    }

    #[test]
    fn mean_explored_positions() {
        let stats = Statistics::new(
            vec![true, true, true, false], 
            vec![Duration::new(0, 0); 4],
            vec![1, 2, 3, 10]);

        assert_eq!(stats.mean_explored_positions(), 4);
    }
}
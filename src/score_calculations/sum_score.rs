use crate::traits::CalculateScore;

#[derive(Clone, Debug, PartialEq)]
pub struct SumScore;

impl CalculateScore for SumScore {
    fn calculate_score(&self, scores: &[Option<f32>]) -> f32 {
        scores.iter().flatten().sum::<f32>()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SumScoreFirstN {
    n: usize,
}

impl SumScoreFirstN {
    pub fn new(n: usize) -> Self {
        Self { n }
    }
}

impl CalculateScore for SumScoreFirstN {
    fn calculate_score(&self, scores: &[Option<f32>]) -> f32 {
        scores.iter().take(self.n).flatten().sum::<f32>()
    }
}
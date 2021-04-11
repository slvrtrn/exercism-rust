#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores { scores: scores.to_vec() }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None;
        }
        let mut max: Option<u32> = None;
        for s in &self.scores {
            match max {
                None => { max = Some(*s) }
                Some(m) => { if m < *s { max = Some(*s) } }
            }
        }
        max
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.scores.is_empty() {
            return Vec::new();
        }
        let mut scores = self.scores.clone();
        scores.sort_by(|a, b| b.cmp(a));
        if self.scores.len() < 4 {
            return scores;
        }
        scores.resize(3, 0);
        scores
    }
}

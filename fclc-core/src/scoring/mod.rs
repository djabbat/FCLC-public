use rand::seq::SliceRandom;
use rand::thread_rng;

/// Federated Shapley Value scorer using Monte Carlo permutation sampling.
///
/// For each of `monte_carlo_samples` random permutations of nodes, we compute
/// the marginal contribution of each node when it is added to the coalition
/// formed by all preceding nodes in that permutation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShapleyScorer {
    pub node_count: usize,
    pub monte_carlo_samples: usize,
}

impl ShapleyScorer {
    /// Create a new scorer for `n` nodes.
    /// Uses M = 150 Monte Carlo samples by default (CONCEPT.md §Scoring, PARAMETERS.md).
    pub fn new(n: usize) -> Self {
        let samples = if n <= 5 { 100 } else { 150 };
        Self {
            node_count: n,
            monte_carlo_samples: samples,
        }
    }

    /// Create scorer with explicit sample count.
    pub fn with_samples(n: usize, samples: usize) -> Self {
        Self {
            node_count: n,
            monte_carlo_samples: samples,
        }
    }

    /// Compute Shapley values for all nodes.
    ///
    /// `performance_fn` takes a coalition (subset of node indices) and returns
    /// the model performance metric (e.g. AUC) for that coalition.
    ///
    /// Returns a vector of length `node_count` where each element is the
    /// estimated Shapley value (contribution) of the corresponding node.
    pub fn compute(&self, performance_fn: impl Fn(&[usize]) -> f64) -> Vec<f64> {
        if self.node_count == 0 {
            return Vec::new();
        }

        let mut shapley = vec![0.0f64; self.node_count];
        let mut rng = thread_rng();
        let mut perm: Vec<usize> = (0..self.node_count).collect();

        for _ in 0..self.monte_carlo_samples {
            perm.shuffle(&mut rng);

            let mut coalition: Vec<usize> = Vec::with_capacity(self.node_count);
            let mut prev_value = performance_fn(&[]);

            for &node in &perm {
                coalition.push(node);
                let new_value = performance_fn(&coalition);
                shapley[node] += new_value - prev_value;
                prev_value = new_value;
            }
        }

        // Average over all samples
        let n_samples = self.monte_carlo_samples as f64;
        for v in shapley.iter_mut() {
            *v /= n_samples;
        }

        shapley
    }

    /// Normalise Shapley values so they sum to 1.0 (relative contributions).
    pub fn normalise(values: &[f64]) -> Vec<f64> {
        let total: f64 = values.iter().sum();
        if total == 0.0 {
            return vec![1.0 / values.len() as f64; values.len()];
        }
        values.iter().map(|v| v / total).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple additive performance function for testing: returns the count of
    /// nodes in the coalition divided by total nodes.
    fn additive_perf(coalition: &[usize]) -> f64 {
        coalition.len() as f64
    }

    #[test]
    fn test_shapley_symmetry() {
        // For an additive game, each player has equal Shapley value = 1.0
        let scorer = ShapleyScorer::with_samples(4, 500);
        let values = scorer.compute(additive_perf);
        assert_eq!(values.len(), 4);
        for v in &values {
            // Should be close to 1.0 with enough samples
            assert!((v - 1.0).abs() < 0.2, "Expected ~1.0, got {v}");
        }
    }

    #[test]
    fn test_shapley_empty() {
        let scorer = ShapleyScorer::new(0);
        let values = scorer.compute(additive_perf);
        assert!(values.is_empty());
    }

    #[test]
    fn test_normalise() {
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let norm = ShapleyScorer::normalise(&values);
        let sum: f64 = norm.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }
}

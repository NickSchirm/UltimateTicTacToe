//! # Contains the [TwoPointCrossover] struct
use rand::distributions::Uniform;
use rand::Rng;
use crate::genetic_algorithm::gene::Gene;
use crate::genetic_algorithm::recombination::Recombination;

/// # Recombination that uses two point crossover
/// 
/// This recombination chooses two random points in the genes and swaps the values between those points.
/// 
/// The points are chosen randomly from a uniform distribution.
pub struct TwoPointCrossover {}

impl Recombination for TwoPointCrossover {
	fn recombine(&self, lhs: Gene, rhs: Gene) -> (Gene, Gene) {
		let mut rng = rand::thread_rng();
		let between = Uniform::from(0..lhs.get_values().len());
		
		let crossover_point1 = rng.sample(between);
		let crossover_point2 = rng.sample(between);
		
		let (start, end) = if crossover_point1 < crossover_point2 {
			(crossover_point1, crossover_point2)
		} else {
			(crossover_point2, crossover_point1)
		};
		
		let mut new_lhs = Vec::new();
		let mut new_rhs = Vec::new();
		
		new_lhs.extend_from_slice(&lhs.get_values()[0..start]);
		new_lhs.extend_from_slice(&rhs.get_values()[start..end]);
		new_lhs.extend_from_slice(&lhs.get_values()[end..]);
		
		new_rhs.extend_from_slice(&rhs.get_values()[0..start]);
		new_rhs.extend_from_slice(&lhs.get_values()[start..end]);
		new_rhs.extend_from_slice(&rhs.get_values()[end..]);
		
		(Gene::with_values(new_lhs), Gene::with_values(new_rhs))
	}
}
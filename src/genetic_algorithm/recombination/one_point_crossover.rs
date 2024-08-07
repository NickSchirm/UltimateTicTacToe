use rand::distributions::Uniform;
use crate::genetic_algorithm::gene::Gene;
use crate::genetic_algorithm::recombination::Recombination;
use rand::Rng;

pub struct OnePointCrossover {}

impl Recombination for OnePointCrossover {
    fn recombine(&self, lhs: Gene, rhs: Gene) -> (Gene, Gene) {
        let mut rng = rand::thread_rng();
        let between = Uniform::from(0..lhs.get_values().len());

        let crossover_point = rng.sample(between);
        
        let mut new_lhs = Vec::new();
        let mut new_rhs = Vec::new();
        
        new_lhs.extend_from_slice(&lhs.get_values()[0..crossover_point]);
        new_lhs.extend_from_slice(&rhs.get_values()[crossover_point..]);
        
        new_rhs.extend_from_slice(&rhs.get_values()[0..crossover_point]);
        new_rhs.extend_from_slice(&lhs.get_values()[crossover_point..]);
        
        (Gene::with_values(new_lhs), Gene::with_values(new_rhs))
    }
}

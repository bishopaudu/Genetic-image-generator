use rand::Rng;

use crate::{image_canvas::Genome, tournamant_selection::ScoredGenome};

// Uniform crossover — for each shape position,
// randomly pick from either parent A or parent B

pub fn crossover(parent_a: &Genome, parent_b: &Genome, rng: &mut impl Rng) -> Genome {
    // we check if both parent have the same number of shapes, otherwise we can't do crossover
    assert_eq!(parent_a.shapes.len(), parent_b.shapes.len(), "Parents must have the same number of shapes for crossover");
    let child_shapes = parent_a.shapes
    .iter()
    .zip(parent_b.shapes.iter())
    .map(|(shape_a, shape_b)| {
        // 50/50 chance of inheriting from either parent
        if rng.gen_bool(0.5) {
            shape_a.clone()
        } else {
            shape_b.clone()
        }
    })
    .collect();
    Genome { shapes: child_shapes }

}

// This function will create the next generation of genomes by performing selection, crossover, and mutation.
pub fn create_next_generation(parents:&[&ScoredGenome],elite:&ScoredGenome,population_size:usize,rng:&mut impl Rng) -> Vec<Genome> {
   // filling the rest with children created by crossover of the selected parents
   let mut next_generation = vec![elite.genome.clone()]; // Start with the elite genome
    while next_generation.len() < population_size {
        //picking 2 random parents from the selected parents
        let parent_a = &parents[rng.gen_range(0..parents.len())].genome;
        let parent_b = &parents[rng.gen_range(0..parents.len())].genome;
        // creating a child by combining the two parents using crossover
        let child = crossover(parent_a, parent_b, rng);
        next_generation.push(child);
    }
    next_generation
}
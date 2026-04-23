use image::{ImageBuffer, Rgba};
use rand::Rng;

use crate::{fitness::compute_fitness, image::Genome, image_render::render};

/// A genome paired with its fitness error score
/// Lower error = better fitness
#[derive(Clone)]
pub struct ScoredGenome {
    pub genome: Genome,
    pub error:  f64,
}

pub type Canvas = ImageBuffer<Rgba<u8>, Vec<u8>>;


/// Run one tournament — pick `tournament_size` random genomes,
/// return the one with the lowest error (the best one)
pub fn tournament_select<'a>(
    population: &'a [ScoredGenome],
    tournament_size: usize,
    rng: &mut impl Rng,
) -> &'a ScoredGenome {

    // Pick `tournament_size` random indices from the population
    let mut best: Option<&ScoredGenome> = None;

    for _ in 0..tournament_size {
        // Pick a random genome from the population
        let idx       = rng.gen_range(0..population.len());
        let candidate = &population[idx];

        best = Some(match best {
            // No best yet — this candidate becomes the best
            None => candidate,

            // We have a best — keep whichever has lower error
            Some(current_best) => {
                if candidate.error < current_best.error {
                    candidate
                } else {
                    current_best
                }
            }
        });
    }

    // safe to unwrap — tournament_size is always > 0
    best.unwrap()
}

/// Select `num_parents` genomes from the population using tournament selection
/// Returns a Vec of references to the winning genomes
pub fn select_parents<'a>(
    population: &'a [ScoredGenome],
    num_parents: usize,
    tournament_size: usize,
    rng: &mut impl Rng,
) -> Vec<&'a ScoredGenome> {
    (0..num_parents)
        .map(|_| tournament_select(population, tournament_size, rng))
        .collect()
}

/// Always keep the single best genome unchanged into the next generation
/// This is called "elitism" — guarantees we never lose our best solution
pub fn get_elite(population: &[ScoredGenome]) -> &ScoredGenome {
    population
        .iter()
        .min_by(|a, b| a.error.partial_cmp(&b.error).unwrap())
        .expect("Population must not be empty")
}

/// Score an entire population — render each genome and compute its error
pub fn score_population(
    population: &[Genome],
    target: &Canvas,
    width: u32,
    height: u32,
) -> Vec<ScoredGenome> {
    population
        .iter()
        .map(|genome| {
            let canvas = render(genome, width, height);
            let error  = compute_fitness(&canvas, target);
            ScoredGenome { genome: genome.clone(), error }
        })
        .collect()
}
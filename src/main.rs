use crate::{evolution::{EvolutionConfig, evolve,}, fitness::create_test_target, image_render::{render, save_image}};
mod image_render;
mod image;
mod fitness;
mod tournamant_selection;
mod crossover;
mod mutation;
mod evolution;
mod display;

fn main() {
    let mut rng = rand::thread_rng();

    // ── Load or create target ──
    let target = create_test_target(200, 200);
    // To use a real photo instead:
    // let target = image::open("your_photo.png")
    //     .expect("Could not load image")
    //     .to_rgba8();

    save_image(&target, "target.png");
    println!("╔════════════════════════════════════╗");
    println!("║   Genetic Image Evolution  v1.0    ║");
    println!("╚════════════════════════════════════╝\n");
    println!("Target: {}×{} pixels", target.width(), target.height());

    let config = EvolutionConfig {
        width:           target.width(),
        height:          target.height(),
        num_generations: 5000,
        save_every:      500,
        display_every:   10,
        ..EvolutionConfig::default()
    };

    let best = evolve(&target, &config, &mut rng);

    // ── Save final result ──
    let final_canvas = render(&best, config.width, config.height);
    save_image(&final_canvas, "final_result.png");

    println!("Final result saved to final_result.png");
    println!("Progress images saved in output/");

    // ── Keep window open after evolution finishes ──
    println!("Window showing final result. Close it or press ESC to exit.");
}
/*fn main() {
    let mut rng = rand::thread_rng();

    // ── Create or load target image ──
    // To use your own image, replace this with:
    // let target = image::open("your_photo.png")
    //     .expect("Could not load image")
    //     .to_rgba8();
    let target = create_test_target(200, 200);
    save_image(&target, "target.png");
    println!("Target saved to target.png");
    println!("Starting evolution...\n");

    // ── Configure the algorithm ──
    let config = EvolutionConfig {
        num_generations: 2000,
        save_every:      200,
        ..EvolutionConfig::default()  // use defaults for everything else
    };

    // ── Run evolution ──
    let best_genome = evolve(&target, &config, &mut rng);

    // ── Save the final result ──
    let final_canvas = render(&best_genome, config.width, config.height);
    save_image(&final_canvas, "final_result.png");
    println!("\nFinal result saved to final_result.png");
    println!("Progress images saved in the output/ folder");
}*/


/*fn main() {
    let mut rng = rand::thread_rng();
      let imagerenderwidth     = 200u32;
    let imagerenderheight    = 200u32;
    let width      = 200.0; // canvas width in pixels
    let height     = 200.0; // canvas height in pixels
    let num_shapes = 10;    // shapes per genome (we'll use 50 later)
    let pop_size   = 5;     // population size (we'll use 100 later)

    let population = random_population(&mut rng, width, height, num_shapes, pop_size);
     let target = create_test_target(imagerenderwidth, imagerenderheight);
         let scored = score_population(&population, &target, imagerenderwidth, imagerenderheight);

    target.save("target.png").expect("Could not save target");
    println!("Saved target.png");

    // Print a summary of what we made
 //   println!("Generated {} genomes, each with {} shapes:\n", pop_size, num_shapes);

    /*for (i, genome) in population.iter().enumerate() {
        println!("  Genome #{}: {} shapes", i, genome.shapes.len());
        for (j, shape) in genome.shapes.iter().enumerate() {
            match shape {
                Shape::Circle { x, y, radius, color } => {
                    println!(
                        "    Shape {}: Circle  at ({:.0},{:.0})  r={:.0}  rgba=({},{},{},{})",
                        j, x, y, radius, color.r, color.g, color.b, color.a
                    );
                }
                Shape::Triangle { x1, y1, x2, y2, x3, y3, color } => {
                    println!(
                        "    Shape {}: Triangle ({:.0},{:.0}) ({:.0},{:.0}) ({:.0},{:.0})  rgba=({},{},{},{})",
                        j, x1, y1, x2, y2, x3, y3, color.r, color.g, color.b, color.a
                    );
                }
            }
        }
        println!();
    }*/


    /*  for (i, genome) in population.iter().enumerate() {
        let canvas = render(genome, imagerenderwidth, imagerenderheight);
        save_image(&canvas, &format!("genome_{}.png", i));
    }
    println!("\nOpen the .png files to see your random genomes!");

     // ── Render and score each genome ──
    println!("\nScoring {} random genomes:\n", pop_size);

    let mut scored: Vec<(f64, usize)> = population
        .iter()
        .enumerate()
        .map(|(i, genome)| {
            let canvas  = render(genome, imagerenderwidth, imagerenderheight);
            let error   = compute_fitness(&canvas, &target);
            let percent = fitness_percentage(error, imagerenderwidth, imagerenderheight);
            println!("  Genome #{}: error = {:.0}  fitness = {:.2}%", i, error, percent);
            (error, i)
        })
        .collect();

    // ── Sort by error — lowest error = best genome ──
    scored.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    println!("\nBest genome: #{} with fitness {:.2}%",
        scored[0].1,
        fitness_percentage(scored[0].0, imagerenderwidth, imagerenderheight)
    );

    // Save the best genome as an image
    let best_genome  = &population[scored[0].1];
    let best_canvas  = render(best_genome, imagerenderwidth, imagerenderheight);
    save_image(&best_canvas, "best.png");*/

     // ── Show all scores ──
    println!("All genomes scored:\n");
    for (i, sg) in scored.iter().enumerate() {
        println!("  Genome #{:2}: fitness = {:.2}%",
            i,
            fitness_percentage(sg.error, imagerenderwidth, imagerenderheight)
        );
    }

    // ── Show the elite ──
    let elite = get_elite(&scored);
    println!("\nElite genome fitness: {:.2}%",
        fitness_percentage(elite.error, imagerenderwidth, imagerenderheight)
    );

    // ── Run tournament selection ──
    let tournament_size = 5;
    let num_parents     = 10;
    let parents = select_parents(&scored, num_parents, tournament_size, &mut rng);

    println!("\nSelected {} parents via tournament selection:", num_parents);
    for (i, parent) in parents.iter().enumerate() {
        println!("  Parent #{}: fitness = {:.2}%",
            i,
            fitness_percentage(parent.error, imagerenderwidth, imagerenderheight)
        );
    }

    // Save the elite genome
    let elite_canvas = render(&elite.genome, imagerenderwidth, imagerenderheight);
    save_image(&elite_canvas, "elite.png");

}*/

/*fn main() {
    let mut rng    = rand::thread_rng();
    let width      = 200u32;
    let height     = 200u32;
    let num_shapes = 50;
    let pop_size   = 20;

    // Target image 
    let target = create_test_target(width, height);
    target.save("target.png").expect("Could not save target");

    //  Generation 0: random population 
    let population = random_population(
        &mut rng, width as f32, height as f32, num_shapes, pop_size
    );
    let scored = score_population(&population, &target, width, height);

    // Show generation 0 best 
    let elite = get_elite(&scored);
    println!("Generation 0 best fitness: {:.2}%",
        fitness_percentage(elite.error, width, height)
    );

    // ── Select parents ──
    let parents = select_parents(&scored, pop_size, 5, &mut rng);

    // ── Create generation 1 via crossover ──
    let next_gen = create_next_generation(&parents, elite, pop_size, &mut rng);

    // ── Score generation 1 ──
    let scored_next = score_population(&next_gen, &target, width, height);
    let elite_next  = get_elite(&scored_next);

    println!("Generation 1 best fitness: {:.2}%",
        fitness_percentage(elite_next.error, width, height)
    );

    // ── Verify child inherited shapes from both parents ──
    // Pick the first two parents and their child and count
    // how many shapes came from each parent
    /*let pa = &parents[0].genome;
    let pb = &parents[1].genome;
    let child = crossover(pa, pb, &mut rng);

    let from_a = pa.shapes.iter()
        .zip(child.shapes.iter())
        .filter(|(a, c)| {
            // Compare shapes by their debug representation
            // (a simple way to check equality for now)
            format!("{:?}", a) == format!("{:?}", c)
        })
        .count();

    println!("\nCrossover test:");
    println!("  Shapes inherited from parent A: {}", from_a);
    println!("  Shapes inherited from parent B: {}", num_shapes - from_a);*/

// ── Show mutation effect on one genome ──
    println!("\nMutation effect on one genome:");
    let mut test_genome = random_genome(
        &mut rng, width as f32, height as f32, num_shapes
    );

    // Render before mutation
    let before_canvas  = render(&test_genome, width, height);
    let before_error   = compute_fitness(&before_canvas, &target);

    // Mutate it
    mutate_genome(&mut test_genome, 0.02, width as f32, height as f32, &mut rng);

    // Render after mutation
    let after_canvas = render(&test_genome, width, height);
    let after_error  = compute_fitness(&after_canvas, &target);

    println!("  Before: {:.2}%", fitness_percentage(before_error, width, height));
    println!("  After:  {:.2}%", fitness_percentage(after_error, width, height));
    println!("  Change: {:+.2}%",
        fitness_percentage(after_error, width, height) -
        fitness_percentage(before_error, width, height)
    );

    // Save outputs
    save_image(&before_canvas, "before_mutation.png");
    save_image(&after_canvas, "after_mutation.png");
    save_image(
        &render(&elite_next.genome, width, height),
        "gen1_best.png"
    );
    // Save generation 1 best
    //let best_canvas = render(&elite_next.genome, width, height);
    //save_image(&best_canvas, "gen1_best.png");
}*/
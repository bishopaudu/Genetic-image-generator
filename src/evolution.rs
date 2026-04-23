use std::time::Instant;

use image::{ImageBuffer, Rgba};
use imageproc::{drawing, point::Point};
use minifb::{Key, Window, WindowOptions};
use rand::Rng;

use crate::{crossover::create_next_generation, display::{canvas_to_buffer, print_progress, side_by_side_buffer}, fitness::fitness_percentage, image::{Genome, random_population}, image_render::{blank_canvas, render, save_image}, mutation::mutate_population, tournamant_selection::{get_elite, score_population, select_parents}};
pub type Canvas = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub struct EvolutionConfig {
    pub width:           u32,
    pub height:          u32,
    pub num_shapes:      usize,
    pub population_size: usize,
    pub tournament_size: usize,
    pub mutation_rate:   f64,
    pub num_generations: usize,
    pub save_every:      usize,
    pub output_dir:      String,
    pub display_every:   usize,  // ← NEW: update window every N generations
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        EvolutionConfig {
            width:           200,
            height:          200,
            num_shapes:      50,
            population_size: 100,
            tournament_size: 5,
            mutation_rate:   0.02,
            num_generations: 5000,
            save_every:      500,
            output_dir:      "output".to_string(),
            display_every:   10,  // update display every 10 generations
        }
    }
}

// ═══════════════════════════════════════════════════════
// EVOLUTION LOOP  ← UPDATED with live display
// ═══════════════════════════════════════════════════════

pub fn evolve(
    target: &Canvas,
    config: &EvolutionConfig,
    rng: &mut impl Rng,
) -> Genome {

    // ── Setup output directory ──
    std::fs::create_dir_all(&config.output_dir)
        .expect("Could not create output directory");

    // ── Create the live display window ──
    // Window is twice the width — left=target, right=evolving image
    let window_width  = config.width  as usize * 2;
    let window_height = config.height as usize;

    let mut window = Window::new(
        "Genetic Image Evolution  │  LEFT: Target  │  RIGHT: Current Best",
        window_width,
        window_height,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    ).expect("Could not create window");

    // Don't run faster than 60fps — prevents burning CPU on display
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // ── Show target image immediately ──
    let target_buffer = canvas_to_buffer(target);
    let (initial_buffer, _, _) = side_by_side_buffer(
        target,
        &blank_canvas(config.width, config.height),
    );
    window.update_with_buffer(&initial_buffer, window_width, window_height)
          .expect("Could not update window");

    // ── Generation 0: random population ──
    println!("Creating random population of {} genomes...", config.population_size);
    let mut population = random_population(
        rng,
        config.width  as f32,
        config.height as f32,
        config.num_shapes,
        config.population_size,
    );

    // ── Score generation 0 ──
    let mut scored     = score_population(&population, target, config.width, config.height);
    let mut best_error = get_elite(&scored).error;
    let mut best_canvas = render(
        &get_elite(&scored).genome,
        config.width,
        config.height,
    );

    println!("Generation 0 fitness: {:.2}%",
        fitness_percentage(best_error, config.width, config.height)
    );
    println!("\nEvolution running — press ESC to stop early\n");

    // ── The main loop ──
    let start_time = Instant::now();

    for generation in 1..=config.num_generations {

        // ── Check if window was closed ──
        // This lets the user press ESC or close the window to stop
        if !window.is_open() || window.is_key_down(Key::Escape) {
            println!("\nStopped early at generation {}", generation);
            break;
        }

        // STEP 1: Elite
        let elite = get_elite(&scored);

        // STEP 2: Selection
        let parents = select_parents(
            &scored,
            config.population_size,
            config.tournament_size,
            rng,
        );

        // STEP 3: Crossover
        let mut next_population = create_next_generation(
            &parents,
            elite,
            config.population_size,
            rng,
        );

        // STEP 4: Mutation
        mutate_population(
            &mut next_population,
            config.mutation_rate,
            config.width  as f32,
            config.height as f32,
            rng,
        );

        // STEP 5: Score
        scored = score_population(&next_population, target, config.width, config.height);
        population = next_population;

        // ── Track improvement ──
        let elite       = get_elite(&scored);
        let fitness_pct = fitness_percentage(elite.error, config.width, config.height);
        let improved    = elite.error < best_error;

        if improved {
            best_error  = elite.error;
            best_canvas = render(&elite.genome, config.width, config.height);
        }

        let elapsed      = start_time.elapsed().as_secs_f32();
        let gens_per_sec = generation as f32 / elapsed;
        let best_pct     = fitness_percentage(best_error, config.width, config.height);

        // ── Update terminal display every generation ──
        print_progress(
            generation,
            config.num_generations,
            fitness_pct,
            best_pct,
            gens_per_sec,
            improved,
        );

        // ── Update live window every N generations ──
        if generation % config.display_every == 0 {
            let (buffer, bw, bh) = side_by_side_buffer(target, &best_canvas);
            window.update_with_buffer(&buffer, bw, bh)
                  .expect("Could not update window");
        }

        // ── Save image every N generations ──
        if generation % config.save_every == 0 {
            let path = format!("{}/gen_{:05}.png", config.output_dir, generation);
            save_image(&best_canvas, &path);
            println!("\n  → Saved {}", path);
        }
    }

    println!("\n\nEvolution complete!");
    println!("Final fitness: {:.2}%",
        fitness_percentage(best_error, config.width, config.height)
    );

    get_elite(&scored).genome.clone()
}

// ═══════════════════════════════════════════════════════
// TARGET IMAGE HELPER
// ═══════════════════════════════════════════════════════

fn create_test_target(width: u32, height: u32) -> Canvas {
    let mut canvas = blank_canvas(width, height);
    drawing::draw_filled_circle_mut(
        &mut canvas,
        (width as i32 / 2, height as i32 / 2),
        60i32,
        Rgba([255u8, 50u8, 50u8, 255u8]),
    );
    let points = vec![
        Point::new(10i32,            (height as i32) - 10),
        Point::new(width  as i32 / 2, 10i32),
        Point::new((width as i32) - 10, (height as i32) - 10),
    ];
    drawing::draw_polygon_mut(
        &mut canvas,
        &points,
        Rgba([50u8, 50u8, 255u8, 255u8]),
    );
    canvas
}



/*pub struct EvolutionConfig {
    pub width:           u32,
    pub height:          u32,
    pub num_shapes:      usize,
    pub population_size: usize,
    pub tournament_size: usize,
    pub mutation_rate:   f64,
    pub num_generations: usize,
    pub save_every:      usize,  // save a progress image every N generations
    pub output_dir:      String, // folder to save images into
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        EvolutionConfig {
            width:           200,
            height:          200,
            num_shapes:      50,
            population_size: 100,
            tournament_size: 5,
            mutation_rate:   0.02,
            num_generations: 5000,
            save_every:      100,
            output_dir:      "output".to_string(),
        }
    }
}
pub type Canvas = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// Run the full genetic algorithm
/// Returns the best genome found
pub fn evolve(
    target: &Canvas,
    config: &EvolutionConfig,
    rng: &mut impl Rng,
) -> Genome {

    // ── Create output directory ──
    std::fs::create_dir_all(&config.output_dir)
        .expect("Could not create output directory");

    // ── Generation 0: random population ──
    println!("Creating random population...");
    let mut population = random_population(
        rng,
        config.width  as f32,
        config.height as f32,
        config.num_shapes,
        config.population_size,
    );

    // ── Score generation 0 ──
    let mut scored = score_population(&population, target, config.width, config.height);
    let mut best_error = get_elite(&scored).error;

    println!("Generation 0 fitness: {:.2}%\n",
        fitness_percentage(best_error, config.width, config.height)
    );

    // ── The main evolution loop ──
    let start_time = Instant::now();

    for generation in 1..=config.num_generations {

        // STEP 1: Find the elite — best genome from last generation
        let elite = get_elite(&scored);

        // STEP 2: Select parents via tournament selection
        let parents = select_parents(
            &scored,
            config.population_size,
            config.tournament_size,
            rng,
        );

        // STEP 3: Create next generation via crossover
        let mut next_population = create_next_generation(
            &parents,
            elite,
            config.population_size,
            rng,
        );

        // STEP 4: Mutate the new generation (skip elite at index 0)
        mutate_population(
            &mut next_population,
            config.mutation_rate,
            config.width  as f32,
            config.height as f32,
            rng,
        );

        // STEP 5: Score the new generation
        scored = score_population(&next_population, target, config.width, config.height);

        // STEP 6: The new generation IS the population now
        population = next_population;

        // ── Track progress ──
        let elite       = get_elite(&scored);
        let fitness_pct = fitness_percentage(elite.error, config.width, config.height);
        let improved    = elite.error < best_error;

        if improved {
            best_error = elite.error;
        }

        // ── Print progress every 10 generations ──
        if generation % 10 == 0 {
            let elapsed = start_time.elapsed().as_secs_f32();
            let gens_per_sec = generation as f32 / elapsed;

            println!(
                "Gen {:5} | Fitness: {:6.2}% | Best: {:6.2}% | {:.1} gen/s {}",
                generation,
                fitness_pct,
                fitness_percentage(best_error, config.width, config.height),
                gens_per_sec,
                if improved { "↑" } else { "" },
            );
        }

        // ── Save progress image every N generations ──
        if generation % config.save_every == 0 {
            let canvas = render(&elite.genome, config.width, config.height);
            let path   = format!("{}/gen_{:05}.png", config.output_dir, generation);
            save_image(&canvas, &path);
            println!("  → Saved {}", path);
        }
    }

    // ── Return the best genome found ──
    get_elite(&scored).genome.clone()
}

// ═══════════════════════════════════════════════════════
// TARGET IMAGE HELPER
// ═══════════════════════════════════════════════════════

fn create_test_target(width: u32, height: u32) -> Canvas {
    let mut canvas = blank_canvas(width, height);

    // Red circle in center
    drawing::draw_filled_circle_mut(
        &mut canvas,
        (width as i32 / 2, height as i32 / 2),
        60i32,
        Rgba([255u8, 50u8, 50u8, 255u8]),
    );

    // Blue triangle
    let points = vec![
        Point::new(10i32,          (height as i32) - 10),
        Point::new(width as i32 / 2, 10i32),
        Point::new((width as i32) - 10, (height as i32) - 10),
    ];
    drawing::draw_polygon_mut(
        &mut canvas,
        &points,
        Rgba([50u8, 50u8, 255u8, 255u8]),
    );

    canvas
}*/

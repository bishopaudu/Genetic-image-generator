use image::{ImageBuffer, Rgba};

/// Convert our Canvas into the pixel format minifb needs
/// minifb wants: Vec<u32> where each u32 is 0x00RRGGBB
/// 
/// 
pub type Canvas = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub fn canvas_to_buffer(canvas: &Canvas) -> Vec<u32> {
    canvas.pixels().map(|p| {
        let r = p[0] as u32;
        let g = p[1] as u32;
        let b = p[2] as u32;
        (r << 16) | (g << 8) | b
    }).collect()
}

/// Create a side-by-side buffer showing target (left) and current best (right)
/// This lets you directly compare the two images at a glance
pub fn side_by_side_buffer(left: &Canvas, right: &Canvas) -> (Vec<u32>, usize, usize) {
    let w = left.width()  as usize;
    let h = left.height() as usize;

    // Total window is twice as wide
    let total_width = w * 2;
    let mut buffer  = vec![0u32; total_width * h];

    // Fill left half with target
    for (i, pixel) in left.pixels().enumerate() {
        let r = pixel[0] as u32;
        let g = pixel[1] as u32;
        let b = pixel[2] as u32;
        let x = i % w;
        let y = i / w;
        buffer[y * total_width + x] = (r << 16) | (g << 8) | b;
    }

    // Fill right half with current best
    for (i, pixel) in right.pixels().enumerate() {
        let r = pixel[0] as u32;
        let g = pixel[1] as u32;
        let b = pixel[2] as u32;
        let x = i % w;
        let y = i / w;
        buffer[y * total_width + (x + w)] = (r << 16) | (g << 8) | b;
    }

    (buffer, total_width, h)
}

/// Print a rich progress bar to the terminal
pub fn print_progress(
    generation: usize,
    num_generations: usize,
    fitness: f64,
    best_ever: f64,
    gens_per_sec: f32,
    improved: bool,
) {
    // ── Progress bar ──
    let bar_width   = 30usize;
    let filled      = (fitness / 100.0 * bar_width as f64) as usize;
    let empty       = bar_width - filled;
    let bar: String = "█".repeat(filled) + &"░".repeat(empty);

    // ── Estimated time remaining ──
    let gens_left  = num_generations - generation;
    let secs_left  = gens_left as f32 / gens_per_sec.max(0.001);
    let mins_left  = (secs_left / 60.0) as u32;
    let secs_left  = (secs_left % 60.0) as u32;

    // ── Print everything on one line ──
    print!(
        "\r Gen {:5}/{} │{}│ {:5.2}% │ Best: {:5.2}% │ {:4.1} gen/s │ ETA: {:2}m{:02}s {}",
        generation,
        num_generations,
        bar,
        fitness,
        best_ever,
        gens_per_sec,
        mins_left,
        secs_left,
        if improved { "↑" } else { " " },
    );

    // Flush stdout so it actually appears immediately
    use std::io::Write;
    std::io::stdout().flush().unwrap();
}
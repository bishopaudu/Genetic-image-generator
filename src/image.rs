use rand::Rng;


#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Debug)]
pub enum Shape {
    Circle {
        x: f32,
        y: f32,
        radius: f32,
        color: Color,
    },
    Triangle {
        x1: f32, y1: f32,
        x2: f32, y2: f32,
        x3: f32, y3: f32,
        color: Color,
    },
}

#[derive(Clone, Debug)]
pub struct Genome {
    pub shapes: Vec<Shape>,
}

// ─────────────────────────────────────────────
// Random generation (Step 4)
// ─────────────────────────────────────────────

/// Generate one random Color
fn random_color(rng: &mut impl Rng) -> Color {
    Color {
        r: rng.gen_range(0..=255),
        g: rng.gen_range(0..=255),
        b: rng.gen_range(0..=255),
        a: rng.gen_range(30..=200), // avoid fully invisible or fully opaque
    }
}

/// Generate one random Shape
/// width and height tell us the canvas size so shapes stay inside the image
pub fn random_shape(rng: &mut impl Rng, width: f32, height: f32) -> Shape {
    // Randomly pick: 0 = Circle, 1 = Triangle
    if rng.gen_bool(0.5) {
        // ── Circle ──
        Shape::Circle {
            x:      rng.gen_range(0.0..width),
            y:      rng.gen_range(0.0..height),
            radius: rng.gen_range(5.0..100.0),
            color:  random_color(rng),
        }
    } else {
        // ── Triangle ──
        Shape::Triangle {
            x1: rng.gen_range(0.0..width),  y1: rng.gen_range(0.0..height),
            x2: rng.gen_range(0.0..width),  y2: rng.gen_range(0.0..height),
            x3: rng.gen_range(0.0..width),  y3: rng.gen_range(0.0..height),
            color: random_color(rng),
        }
    }
}

/// Generate one random Genome (one image's DNA)
/// num_shapes: how many shapes this genome will have (e.g. 50)
pub fn random_genome(rng: &mut impl Rng, width: f32, height: f32, num_shapes: usize) -> Genome {
    let shapes = (0..num_shapes)
        .map(|_| random_shape(rng, width, height))
        .collect(); // collect the iterator into a Vec<Shape>

    Genome { shapes }
}

/// Generate a whole population of random genomes
/// population_size: how many individuals (e.g. 100)
pub fn random_population(
    rng: &mut impl Rng,
    width: f32,
    height: f32,
    num_shapes: usize,
    population_size: usize,
) -> Vec<Genome> {
    (0..population_size)
        .map(|_| random_genome(rng, width, height, num_shapes))
        .collect()
}
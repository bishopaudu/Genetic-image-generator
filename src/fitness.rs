/*use image::Rgba;
use imageproc::{drawing::{self, Canvas}, point::Point};
use crate::image_render::blank_canvas;

/// Compare two canvases pixel by pixel
/// Returns the total error — LOWER means MORE similar
/// We use Sum of Squared Differences (SSD)
pub fn compute_fitness(candidate: &Canvas, target: &Canvas) -> f64 {
    let mut total_error: f64 = 0.0;

    // zip() pairs up pixels from both images simultaneously
    // candidate.pixels() gives us every pixel left-to-right, top-to-bottom
    for (candidate_pixel, target_pixel) in candidate.pixels().zip(target.pixels()) {
        // Each pixel is Rgba([r, g, b, a])
        // We only compare R, G, B — not alpha (alpha is our tool, not the target's)
        let dr = candidate_pixel[0] as f64 - target_pixel[0] as f64; // red diff
        let dg = candidate_pixel[1] as f64 - target_pixel[1] as f64; // green diff
        let db = candidate_pixel[2] as f64 - target_pixel[2] as f64; // blue diff

        // Square each difference and add to total
        total_error += dr * dr + dg * dg + db * db;
    }

    total_error
}

/// The maximum possible error for an image of given dimensions
/// Useful for normalizing fitness to a 0.0–1.0 range
pub fn max_possible_error(width: u32, height: u32) -> f64 {
    // Worst case: every channel differs by 255
    // Per pixel: 255² + 255² + 255² = 195,075
    let pixels = (width * height) as f64;
    pixels * (255.0 * 255.0 * 3.0)
}

/// Returns fitness as a percentage: 100.0 = perfect match, 0.0 = worst possible
pub fn fitness_percentage(error: f64, width: u32, height: u32) -> f64 {
    let max_error = max_possible_error(width, height);
    (1.0 - error / max_error) * 100.0
}

/// Create a simple test target — a red circle on white background
/// Replace this with image::open("your_photo.png") later
pub fn create_test_target(width: u32, height: u32) -> Canvas {
    let mut canvas = blank_canvas(width, height);

    // Draw a red circle in the center
    drawing::draw_filled_circle_mut(
        &mut canvas,
        (width as i32 / 2, height as i32 / 2),
        60i32,
        Rgba([255u8, 50u8, 50u8, 255u8]),
    );

    // Draw a blue triangle
    let points = vec![
        Point::new(10i32, 190i32),
        Point::new(100i32, 10i32),
        Point::new(190i32, 190i32),
    ];
    drawing::draw_polygon_mut(
        &mut canvas,
        &points,
        Rgba([50u8, 50u8, 255u8, 255u8]),
    );

    canvas
}*/

use image::{ImageBuffer, Rgba};
use imageproc::drawing;
use imageproc::point::Point;

use crate::image_render::blank_canvas;

/// Our canvas type used throughout the project
pub type Canvas = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// Compare two canvases pixel by pixel
/// Returns the total error — LOWER means MORE similar
/// We use Sum of Squared Differences (SSD)
pub fn compute_fitness(candidate: &Canvas, target: &Canvas) -> f64 {
    let mut total_error: f64 = 0.0;

    // zip() pairs up pixels from both images simultaneously
    // candidate.pixels() gives us every pixel left-to-right, top-to-bottom
    for (candidate_pixel, target_pixel) in candidate.pixels().zip(target.pixels()) {
        // Each pixel is Rgba([r, g, b, a])
        // We only compare R, G, B — not alpha (alpha is our tool, not the target's)
        let dr = candidate_pixel[0] as f64 - target_pixel[0] as f64;
        let dg = candidate_pixel[1] as f64 - target_pixel[1] as f64;
        let db = candidate_pixel[2] as f64 - target_pixel[2] as f64;

        // Square each difference and add to total
        total_error += dr * dr + dg * dg + db * db;
    }

    total_error
}

/// The maximum possible error for an image of given dimensions
/// Useful for normalizing fitness to a 0.0–1.0 range
pub fn max_possible_error(width: u32, height: u32) -> f64 {
    // Worst case: every channel differs by 255
    // Per pixel: 255² + 255² + 255²
    let pixels = (width * height) as f64;
    pixels * (255.0 * 255.0 * 3.0)
}

/// Returns fitness as a percentage: 100.0 = perfect match, 0.0 = worst possible
pub fn fitness_percentage(error: f64, width: u32, height: u32) -> f64 {
    let max_error = max_possible_error(width, height);
    (1.0 - error / max_error) * 100.0
}

/// Create a simple test target — a red circle on white background
/// Replace this with image::open("your_photo.png") later
pub fn create_test_target(width: u32, height: u32) -> Canvas {
    let mut canvas = blank_canvas(width, height);

    // Draw a red circle in the center
    drawing::draw_filled_circle_mut(
        &mut canvas,
        (width as i32 / 2, height as i32 / 2),
        60,
        Rgba([255, 50, 50, 255]),
    );

    // Draw a blue triangle
    let points = vec![
        Point::new(10, 190),
        Point::new(100, 10),
        Point::new(190, 190),
    ];

    drawing::draw_polygon_mut(
        &mut canvas,
        &points,
        Rgba([50, 50, 255, 255]),
    );

    canvas
}
use image::{ImageBuffer, Rgba};
use imageproc::{drawing, point::Point};
use crate::image::{Genome, Shape};

/// ImageBuffer<Rgba<u8>, Vec<u8>> means:
///   - each pixel is Rgba (4 bytes: red, green, blue, alpha)
///   - the underlying storage is a Vec<u8>
type Canvas = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// Create a blank white canvas
pub fn blank_canvas(width: u32, height: u32) -> Canvas {
    ImageBuffer::from_pixel(width, height, Rgba([255u8, 255u8, 255u8, 255u8]))
}

/// Draw one shape onto a canvas
pub fn draw_shape(canvas: &mut Canvas, shape: &Shape) {
    match shape {
        // ── Draw a circle ──
        Shape::Circle { x, y, radius, color } => {
            drawing::draw_filled_circle_mut(
                canvas,
                (*x as i32, *y as i32),  // center point
                *radius as i32,           // radius
                Rgba([color.r, color.g, color.b, color.a]),
            );
        }

        // ── Draw a triangle ──
        Shape::Triangle { x1, y1, x2, y2, x3, y3, color } => {
            // imageproc draws polygons as a list of Points
            let points = vec![
                Point::new(*x1 as i32, *y1 as i32),
                Point::new(*x2 as i32, *y2 as i32),
                Point::new(*x3 as i32, *y3 as i32),
            ];
            drawing::draw_polygon_mut(
                canvas,
                &points,
                Rgba([color.r, color.g, color.b, color.a]),
            );
        }
    }
}

/// Render a full genome into a Canvas
/// This is the key function — it turns DNA into a visible image
pub fn render(genome: &Genome, width: u32, height: u32) -> Canvas {
    let mut canvas = blank_canvas(width, height);

    // Draw each shape in order — later shapes appear on top of earlier ones
    for shape in &genome.shapes {
        draw_shape(&mut canvas, shape);
    }

    canvas
}

/// Save a canvas to disk as a PNG file
pub fn save_image(canvas: &Canvas, path: &str) {
    canvas.save(path).expect("Failed to save image");
    println!("Saved: {}", path);
}
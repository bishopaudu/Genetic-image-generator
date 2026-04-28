use rand::Rng;

use crate::image_canvas::{Color, Genome, Shape, random_shape};


// How much to nudge a position value (in pixels)
const POSITION_NUDGE : f32 = 20.0;

// How much to nudge a position value (in pixels)
const RADIUS_NUDEGE : f32 = 10.0;

// How much to nudge a color channel value (0-255)
const COLOR_NUDGE:f32 = 30.0;


// Nudge a single f32 value by a random amount
//stays clamped within [min, max]
pub fn nudge_f32(value: f32,amount: f32,min : f32, max:f32,rng: &mut impl Rng) -> f32 {
    let detla = rng.gen_range(-amount..=amount);
    (value + detla).clamp(min, max)

}

//Nudge a single u8 color channel
fn nudge_u8(value: u8, amount: f32, rng: &mut impl Rng) -> u8 {
    let delta = rng.gen_range(-amount..=amount);
    ((value as f32) + delta).clamp(0.0, 255.0) as u8
}

// Mutate a color by nudging each channel slightly
fn mutate_color(color: &mut Color, rng: &mut impl Rng) {
    color.r = nudge_u8(color.r, COLOR_NUDGE, rng);
    color.g = nudge_u8(color.g, COLOR_NUDGE, rng);
    color.b = nudge_u8(color.b, COLOR_NUDGE, rng);
    color.a = nudge_u8(color.a, COLOR_NUDGE, rng);
}

pub fn mutate_shape(    shape:&mut Shape, width:f32, height:f32, rng: &mut impl Rng){
    if rng.gen_bool(0.1){
        *shape = random_shape(rng,width,height);
        return;
    }
        // 90% chance: tweak the existing shape's value
    match  shape {
        Shape::Circle { x, y, radius, color } => {
            if rng.gen_bool(0.5){
                *x = nudge_f32(*x, POSITION_NUDGE, 0.0, width, rng);
            }
            if rng.gen_bool(0.5){
                *y = nudge_f32(*y, POSITION_NUDGE, 0.0, height, rng);
            }
            if rng.gen_bool(0.5){
                *radius = nudge_f32(*radius, RADIUS_NUDEGE, 5.0, 100.0, rng);
            }
            if rng.gen_bool(0.5){
                mutate_color(color, rng);
            }
        }
        Shape::Triangle { x1, y1, x2, y2, x3, y3, color } => {
            if rng.gen_bool(0.5){
                *x1 = nudge_f32(*x1, POSITION_NUDGE, 0.0, width, rng);
                *y1 = nudge_f32(*y1, POSITION_NUDGE, 0.0, height, rng);
            }
            if rng.gen_bool(0.5){
                *x2 = nudge_f32(*x2, POSITION_NUDGE, 0.0, width, rng);
                *y2 = nudge_f32(*y2, POSITION_NUDGE, 0.0, height, rng);
            }
            if rng.gen_bool(0.5){
                *x3 = nudge_f32(*x3, POSITION_NUDGE, 0.0, width, rng);
                *y3 = nudge_f32(*y3, POSITION_NUDGE, 0.0, height, rng);
            }
            if rng.gen_bool(0.5){
                mutate_color(color,rng);
            }
        }
          
    } 
}

//Mutate an entire genome — each shape has a chance of being mutated
pub fn mutate_genome(genome:&mut Genome,mutation_rate: f64,width:f32,height:f32,rng: &mut impl Rng){
    for shape in  genome.shapes.iter_mut() {
        if rng.gen_bool(mutation_rate) {
            mutate_shape(shape, width, height, rng);
        }
    }
}

// Mutate every genome in a list
// Note: we skip index 0 because that's the elite — never mutate it!
pub fn mutate_population(population:&mut Vec<Genome>,mutation_rate:f64,width:f32,height:f32,rng:&mut impl Rng){
 for genome in population.iter_mut().skip(1){
    mutate_genome(genome, mutation_rate, width, height, rng);
 }
}
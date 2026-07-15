use std::hint::black_box;
use std::time::Instant;

use pancanga_engine::core::math::{atan2_deg, cos_deg, normalize_360, sin_deg};
use pancanga_engine::core::Degrees;

const ITERATIONS: usize = 5_000_000;

fn main() {
    run_degrees_benchmark("normalize_360", |angle| normalize_360(angle).value());
    run_degrees_benchmark("sin_deg", sin_deg);
    run_degrees_benchmark("cos_deg", cos_deg);
    run_atan2_benchmark();
}

fn run_degrees_benchmark(name: &str, function: impl Fn(Degrees) -> f64) {
    let started_at = Instant::now();
    let mut accumulator = 0.0;

    for index in 0..ITERATIONS {
        let angle = Degrees::new((index as f64) - 720.0);
        accumulator += black_box(function(black_box(angle)));
    }

    let elapsed = started_at.elapsed();

    println!("{name}: {ITERATIONS} iterations in {elapsed:?} ({accumulator})");
}

fn run_atan2_benchmark() {
    let started_at = Instant::now();
    let mut accumulator = 0.0;

    for index in 0..ITERATIONS {
        let y = ((index % 360) as f64) - 180.0;
        let x = (((index + 90) % 360) as f64) - 180.0;
        accumulator += black_box(atan2_deg(black_box(y), black_box(x)).value());
    }

    let elapsed = started_at.elapsed();

    println!("atan2_deg: {ITERATIONS} iterations in {elapsed:?} ({accumulator})");
}

use std::hint::black_box;
use std::time::Instant;

use pancanga_engine::astronomy::algorithms::{
    apparent_solar_longitude, apply_solar_aberration, earth_heliocentric_longitude,
    fundamental_lunar_arguments, julian_centuries, nutation_in_longitude,
};
use pancanga_engine::core::{AstronomicalUnits, Degrees, JulianDate};

const RELEASE_ITERATIONS: usize = 10_000_000;
const DEBUG_ITERATIONS: usize = 10_000;

fn main() {
    run_julian_centuries_benchmark();
    run_earth_heliocentric_longitude_benchmark();
    run_solar_aberration_benchmark();
    run_nutation_in_longitude_benchmark();
    run_apparent_solar_longitude_benchmark();
    run_fundamental_lunar_arguments_benchmark();
}

fn run_julian_centuries_benchmark() {
    let iterations = benchmark_iterations();
    let started_at = Instant::now();
    let mut accumulator = 0.0;

    for index in 0..iterations {
        let jd = JulianDate::new(2_451_545.0 + index as f64);
        accumulator += black_box(julian_centuries(black_box(jd)));
    }

    let elapsed = started_at.elapsed();

    println!("julian_centuries: {iterations} iterations in {elapsed:?} ({accumulator})");
}

fn run_earth_heliocentric_longitude_benchmark() {
    let iterations = benchmark_iterations();
    let started_at = Instant::now();
    let mut accumulator = 0.0;

    for index in 0..iterations {
        let jd = JulianDate::new(2_451_545.0 + index as f64);
        accumulator += black_box(earth_heliocentric_longitude(black_box(jd))).value();
    }

    let elapsed = started_at.elapsed();

    println!(
        "earth_heliocentric_longitude: {iterations} iterations in {elapsed:?} ({accumulator})"
    );
}

fn run_solar_aberration_benchmark() {
    let iterations = benchmark_iterations();
    let started_at = Instant::now();
    let mut accumulator = 0.0;

    for index in 0..iterations {
        let longitude = Degrees::new((index as f64).rem_euclid(360.0));
        let radius = AstronomicalUnits::new(0.983 + (index % 35) as f64 * 0.001);
        accumulator += black_box(apply_solar_aberration(
            black_box(longitude),
            black_box(radius),
        ))
        .value();
    }

    let elapsed = started_at.elapsed();

    println!("apply_solar_aberration: {iterations} iterations in {elapsed:?} ({accumulator})");
}

fn run_nutation_in_longitude_benchmark() {
    let iterations = benchmark_iterations();
    let started_at = Instant::now();
    let mut accumulator = 0.0;

    for index in 0..iterations {
        let jd = JulianDate::new(2_451_545.0 + index as f64);
        accumulator += black_box(nutation_in_longitude(black_box(jd))).value();
    }

    let elapsed = started_at.elapsed();

    println!("nutation_in_longitude: {iterations} iterations in {elapsed:?} ({accumulator})");
}

fn run_apparent_solar_longitude_benchmark() {
    let iterations = benchmark_iterations();
    let started_at = Instant::now();
    let mut accumulator = 0.0;

    for index in 0..iterations {
        let longitude = Degrees::new((index as f64).rem_euclid(360.0));
        let aberration = Degrees::new(-0.0057);
        let nutation = Degrees::new(((index % 21) as f64 - 10.0) * 0.0001);
        accumulator += black_box(apparent_solar_longitude(
            black_box(longitude),
            black_box(aberration),
            black_box(nutation),
        ))
        .value();
    }

    let elapsed = started_at.elapsed();

    println!("apparent_solar_longitude: {iterations} iterations in {elapsed:?} ({accumulator})");
}

fn run_fundamental_lunar_arguments_benchmark() {
    let iterations = benchmark_iterations();
    let started_at = Instant::now();
    let mut accumulator = 0.0;

    for index in 0..iterations {
        let jd = JulianDate::new(2_451_545.0 + index as f64);
        let arguments = black_box(fundamental_lunar_arguments(black_box(jd)));
        accumulator += arguments.mean_elongation().value()
            + arguments.solar_mean_anomaly().value()
            + arguments.lunar_mean_anomaly().value()
            + arguments.argument_of_latitude().value();
    }

    let elapsed = started_at.elapsed();

    println!("fundamental_lunar_arguments: {iterations} iterations in {elapsed:?} ({accumulator})");
}

fn benchmark_iterations() -> usize {
    if cfg!(debug_assertions) {
        DEBUG_ITERATIONS
    } else {
        RELEASE_ITERATIONS
    }
}

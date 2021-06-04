use rand::prelude::*;

fn main() {
    print_pi(3, 11, std::time::Instant::now())
}

fn print_pi(range_min: u32, range_max: u32, time: std::time::Instant) {
    println!(
        "PI: {}\nCalculated in {} seconds",
        monte_carlo(range_min, range_max),
        time.elapsed().as_secs_f64()
    )
}

fn monte_carlo(range_min: u32, range_max: u32) -> f64 {
    (range_min..range_max)
        .map(|n| calculate_pi(10u64.pow(n)))
        .sum::<f64>()
        / (range_max - range_min) as f64
}

fn calculate_pi(square_dots: u64) -> f64 {
    calculate_circle_dots(
        square_dots,
        rand::distributions::Uniform::new_inclusive(-1f64, 1f64),
        &mut rand::thread_rng(),
    ) as f64
        / square_dots as f64
        * 4f64
}

fn calculate_circle_dots(
    square_dots: u64,
    uniform: rand::distributions::Uniform<f64>,
    rng: &mut rand::prelude::ThreadRng,
) -> u64 {
    (0..square_dots)
        .map(|_| (uniform.sample(rng).powf(2f64) + uniform.sample(rng).powf(2f64) <= 1f64) as u64)
        .sum::<u64>()
}

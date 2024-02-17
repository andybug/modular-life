use std::time::Instant;
use triangle_number::*;

// Calculate the triangle number
fn main() {
    const SIDE_LENGTH: u64 = 67_108_864;

    let series = create_triangle_number_series(SIDE_LENGTH);

    // sum the series and capture how long it took
    let start = Instant::now();
    let sum = sum_series(&series);
    let duration = start.elapsed();

    // compare our summed series against the "real" calculation
    assert_eq!(sum, calculate_triangle_number(SIDE_LENGTH));

    println!("{} µs", duration.as_micros());
}

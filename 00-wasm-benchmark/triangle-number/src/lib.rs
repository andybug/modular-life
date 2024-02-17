/// Create a series that contains the integers from `1` to `side_length`
pub fn create_triangle_number_series(side_length: u64) -> Vec<u64> {
    // `=side_length` makes the range inclusive
    (1..=side_length).collect()
}

/// Sum up all of the values in the series
pub fn sum_series(series: &[u64]) -> u64 {
    series.iter().sum()
}

/// Calculate the triangle number using the formula
///
///  The formaula is `n*(n+1)/2`, where `n` is the side length
pub fn calculate_triangle_number(side_length: u64) -> u64 {
    (side_length * (side_length + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn series_size() {
        let result = create_triangle_number_series(0);
        assert_eq!(result.len(), 0);

        let result = create_triangle_number_series(1);
        assert_eq!(result.len(), 1);

        let result = create_triangle_number_series(16);
        assert_eq!(result.len(), 16);
    }
}

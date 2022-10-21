use colored::Colorize;

// Add reduce util helper function that takes in the vector of numbers
// and a callback  that is passed to the reduce method. Added a Box
// to allocate memory on the heap as the values will not be known at compilation time
pub fn reduce_util(numbers: Vec<f32>, callback: Box<dyn Fn(f32, f32) -> f32>) -> Option<f32> {
    numbers.into_iter().reduce(callback)
}

/// If the result ends with a .0 it returns the result with no decimal
/// otherwise it will return the result with the given or default precision
pub fn check_for_lone_zero_decimal(result: f32, precision: usize) -> f32 {
    if result.fract() == 0.0 {
        format!("{:.1$}", result, 0).parse::<f32>().unwrap()
    } else {
        format!("{:.1$}", result, precision).parse::<f32>().unwrap()
    }
}

pub fn print_result(result: f32) {
    println!("The answer is: {}", format!("{:2}", result).green().bold());
}

#[cfg(test)]
mod tests {
    use super::check_for_lone_zero_decimal;

    #[test]
    pub fn check_lone_zero() {
        assert_eq!(check_for_lone_zero_decimal(20.8, 1), 20.8);

        let actual_string_value = check_for_lone_zero_decimal(20.0, 1).to_string();
        let expected_string_value = "20";

        assert_eq!(actual_string_value, expected_string_value)
    }
}

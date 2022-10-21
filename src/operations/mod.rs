use crate::utils::reduce_util;

pub fn add(numbers: Vec<f32>) -> f32 {
    let result = reduce_util(numbers, Box::new(|a, b| a + b));

    match result {
        Some(result) => result,
        None => panic!("Invalid operation"),
    }
}

pub fn subtract(numbers: Vec<f32>) -> f32 {
    let result = reduce_util(numbers, Box::new(|a, b| a - b));

    match result {
        Some(result) => result,
        None => panic!("Invalid operation"),
    }
}

pub fn multiply(numbers: Vec<f32>) -> f32 {
    let result = reduce_util(numbers, Box::new(|a, b| a * b));

    match result {
        Some(result) => result,
        None => panic!("Invalid operation"),
    }
}

pub fn divide(numbers: Vec<f32>) -> f32 {
    let result = reduce_util(numbers, Box::new(|a, b| a / b));

    match result {
        Some(result) => result,
        None => panic!("Invalid operation"),
    }
}

#[cfg(test)]
mod tests {
    use super::{add, divide, multiply, subtract};

    #[test]
    fn addition() {
        assert_eq!(add(vec![1.0, 201.4, 2.3]), 204.7)
    }
    #[test]
    fn subtraction() {
        assert_eq!(subtract(vec![20.1, 10.3, 0.0]), 9.8)
    }
    #[test]
    fn multiplication() {
        assert_eq!(multiply(vec![3.0, 5.0, 80.0]), 1200.0)
    }

    #[test]
    fn division() {
        assert_eq!(divide(vec![8.0, 2.0, 1.0]), 4.0)
    }
}

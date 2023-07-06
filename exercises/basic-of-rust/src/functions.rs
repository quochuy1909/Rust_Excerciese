// Exercise 1
// Fix all errors
pub fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

//Exercise 2
// Input: Provide an arbitrary value of n
// Implement sum function: 1+2+3+..n
// Output: Calculate sum 1 to n
pub fn sum_one_to_n(mut n: u32) -> u32 {
    let _i = 1;
    for i in 1..n {
        n = n + i;
    }
    return n;
    // your code for summing all digits from 1 to `n` (inclusive) should go
    // here (you can remove the sample return of `0`)
}

// Exercise 3
// Input: list of arbitrary numbers
// Problem: Calculate the average of a list of numbers
// Output: Average Number
fn calculate_average(numbers: &[f64]) -> f64 {
    let mut total = 0 as f64;
    let mut count = 0 as f64;
    for n in numbers.iter() {
        total = total as f64 + n;
        count = count + 1 as f64;
    }
    return total / count;
}

// Exercise 4
// Calculate the sum of all even numbers in a list
fn sum_even_numbers(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for i in numbers.iter() {
        total = total + i;
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 1
    #[test]
    fn sum_should_work() {
        let (x, y) = (1, 2);
        let s = sum(x, y);

        assert_eq!(s, 3);
    }

    // Test for exercise 2
    #[test]
    fn test_sum_0() {
        let result = sum_one_to_n(0);

        assert_eq!(result, 0);
    }

    // Test for exercise 2
    #[test]
    fn test_sum_1() {
        let result = sum_one_to_n(1);

        assert_eq!(result, 1);
    }

    // Test for exercise 2
    #[test]
    fn test_sum_100() {
        let result = sum_one_to_n(100);

        assert_eq!(result, 5050);
    }

    // Test for exercise 3
    #[test]
    fn test_calculate_average() {
        // Test case 1: Non-empty slice
        let numbers = [2.5, 4.8, 6.3, 1.7, 3.9];
        let result = calculate_average(&numbers);
        assert_eq!(result, 3.84);
    }

    // Test for exercise 3
    #[test]
    fn test_calculate_average_empty() {
        // Test case 1: Non-empty slice
        let numbers = [];
        let result = calculate_average(&numbers);
        assert_eq!(result, 0.0);
    }

    // Test for exercise 4
    #[test]
    fn test_sum_even_numbers() {
        assert_eq!(sum_even_numbers(&[1, 2, 3, 4, 5, 6]), 12);
        assert_eq!(sum_even_numbers(&[10, 20, 30, 40, 50]), 150);
        assert_eq!(sum_even_numbers(&[15, 25, 35, 45, 55]), 0);
        assert_eq!(sum_even_numbers(&[-2, 0, 2, 4, 6]), 10);
    }
}

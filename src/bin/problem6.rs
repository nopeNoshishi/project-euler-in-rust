//! Sum Square Difference
//! https://projecteuler.net/problem=6
//! 
//! 数列の仕組みを使って計算
//! 1. 2乗の合計は、n(n+1)(2n+1)/6
//! 2. 連続する和の合計は、n(n+1)/2

fn main() {

    let first = sum_square(100);
    let second = squares_sum(100);

    println!("The Sum of squares: {}", first);
    println!("The square of the sum: {}", second);
    println!("Answer: {}", first - second);
}

fn squares_sum(num: u64) -> u64 {

    num * (num + 1) * (2 * num + 1) / 6
}

fn sum_square(num: u64) -> u64 {
    let sum = num * (num + 1) / 2;

    sum * sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squares_sum() {
        assert_eq!(385, squares_sum(10));
        assert_eq!(338350, squares_sum(100));
    }

    #[test]
    fn test_sum_square() {
        assert_eq!(3025, sum_square(10));
        assert_eq!(25502500, sum_square(100));
    }
}
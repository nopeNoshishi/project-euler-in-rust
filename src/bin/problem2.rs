//! Problem2 Even Fibonacci numbers
//!
//! [方針]
//! フィボナッチ数列を形成してから、偶数フィルターで合計を算出する。
//!

fn main() {
    let sum = sum_even_fibonacci(4_000_000);
    println!("{}", sum);
}

fn sum_even_fibonacci(lim: u64) -> u64 {
    let mut base1 = 1_u64;
    let mut base2 = 2_u64;

    let mut fibo = Vec::<u64>::new();
    fibo.push(base1);
    fibo.push(base2);

    loop {
        if base1 > base2 {
            base2 += base1;

            if base2 > lim {
                break;
            }
            fibo.push(base2);
        } else {
            base1 += base2;

            if base1 > lim {
                break;
            }
            fibo.push(base1);
        }
    }

    fibo.iter().filter(|x| **x % 2 == 0).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_even_fibonacci() {
        assert_eq!(sum_even_fibonacci(100), 44);
        assert_eq!(sum_even_fibonacci(4_000_000), 4613732);
    }
}

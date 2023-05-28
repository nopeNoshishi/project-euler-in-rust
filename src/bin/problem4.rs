//! Problem4 Largest palindrome product
//!
//! 「方針」
//! 回文の判定関数を作成する（折り返して同じであれば回文と判定）。
//! 全ての回文を列挙し最大のものを取り出す。
//! TODO: 全探索しなくても良い方法を検討
//!

fn main() {
    let large_palindrome = search_large_palindrome(3);
    println!("{}", large_palindrome);
}

fn search_large_palindrome(digit: u32) -> u64 {
    let small_num = 10_u64.pow(digit - 1);
    let large_num = 10_u64.pow(digit);

    let mut candidate = Vec::<u64>::new();
    for i in (small_num..large_num).rev() {
        for j in (small_num..large_num).rev().map(|x| x * i) {
            if is_palindrome(&j.to_string()) {
                candidate.push(j)
            }
        }
    }

    *candidate.iter().max().unwrap()
}

fn is_palindrome(target: &str) -> bool {
    let digit = target.len();

    // 回文を分割して、右側を反転する
    let (left, right) = target.as_bytes().split_at(digit / 2);
    let mut right_vec = right.to_vec();
    right_vec.reverse();

    // 回文の桁数に応じて判定が分かれる
    if digit % 2 == 0 {
        left == right_vec
    } else {
        right_vec.pop();
        left == right_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(false, is_palindrome("100000"));
        assert_eq!(false, is_palindrome("1012400"));
        assert_eq!(false, is_palindrome("1200121"));

        assert_eq!(true, is_palindrome("130031"));
        assert_eq!(true, is_palindrome("3219123"));
        assert_eq!(true, is_palindrome("99999999999999"));
    }

    #[test]
    fn test_search() {
        assert_eq!(9009, search_large_palindrome(2));
        assert_eq!(906609, search_large_palindrome(3));
        // assert_eq!(99000099, search_large_palindrome(4)); // めちゃくちゃ遅い！
    }
}

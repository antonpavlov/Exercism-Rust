/// The `raindrops` function converts a number into a string that contains
/// raindrop sounds corresponding to certain potential factors.
/// The rules of raindrops are that if a given number:
///   - has 3 as a factor, add 'Pling' to the result
///   - has 5 as a factor, add 'Plang' to the result
///   - has 7 as a factor, add 'Plong' to the result
///   - does not have any of 3, 5, or 7 as a factor, the result should be the
///     digits of the number.
///
/// # Parameters
///
/// - `n`: An u32 number
///
/// # Returns
///
/// A `result` which is:
///
/// - `"Pling"`: If an input number has 3 as a factor, add 'Pling' to the result
/// - `"Plang"`: If an input number has 5 as a factor, add 'Plang' to the result
/// - `"Plong"`: If an input number has 7 as a factor, add 'Plong' to the result
/// - `"[digits]"`: If an input number does not have any of 3, 5, or 7 as a factor,
///                 the result should be the digits of the number.
///
/// # Examples
///
/// ```rust
/// assert_eq!("Pling", raindrops(3));
/// assert_eq!("PlingPlang", raindrops(15));
/// assert_eq!("52", raindrops(52));
/// assert_eq!("PlingPlangPlong", raindrops(105));
///
/// ```
pub fn raindrops(n: u32) -> String {
    let mut result: String = "".to_owned();
    let pling: &str = "Pling";
    let plang: &str = "Plang";
    let plong: &str = "Plong";

    if (n % 3 == 0) && (n % 5 != 0) && (n % 7 != 0) {
        result.push_str(pling);
    } else if (n % 3 == 0) && (n % 5 == 0) && (n % 7 != 0) {
        result.push_str(pling);
        result.push_str(plang);
    } else if (n % 3 != 0) && (n % 5 == 0) && (n % 7 != 0) {
        result.push_str(plang);
    } else if (n % 3 != 0) && (n % 5 != 0) && (n % 7 == 0) {
        result.push_str(plong);
    } else if (n % 3 == 0) && (n % 5 != 0) && (n % 7 == 0) {
        result.push_str(pling);
        result.push_str(plong);
    } else if (n % 3 != 0) && (n % 5 == 0) && (n % 7 == 0) {
        result.push_str(plang);
        result.push_str(plong);
    } else if (n % 3 == 0) && (n % 5 == 0) && (n % 7 == 0) {
        result.push_str(pling);
        result.push_str(plang);
        result.push_str(plong);
    } else {
        result = format!("{}", n);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!("1", raindrops(1));
    }

    #[test]
    #[ignore]
    fn test_3() {
        assert_eq!("Pling", raindrops(3));
    }

    #[test]
    #[ignore]
    fn test_5() {
        assert_eq!("Plang", raindrops(5));
    }

    #[test]
    #[ignore]
    fn test_7() {
        assert_eq!("Plong", raindrops(7));
    }

    #[test]
    #[ignore]
    fn test_6() {
        assert_eq!("Pling", raindrops(6));
    }

    #[test]
    #[ignore]
    fn test_8() {
        assert_eq!("8", raindrops(8));
    }

    #[test]
    #[ignore]
    fn test_9() {
        assert_eq!("Pling", raindrops(9));
    }

    #[test]
    #[ignore]
    fn test_10() {
        assert_eq!("Plang", raindrops(10));
    }

    #[test]
    #[ignore]
    fn test_14() {
        assert_eq!("Plong", raindrops(14));
    }

    #[test]
    #[ignore]
    fn test_15() {
        assert_eq!("PlingPlang", raindrops(15));
    }

    #[test]
    #[ignore]
    fn test_21() {
        assert_eq!("PlingPlong", raindrops(21));
    }

    #[test]
    #[ignore]
    fn test_25() {
        assert_eq!("Plang", raindrops(25));
    }

    #[test]
    #[ignore]
    fn test_27() {
        assert_eq!("Pling", raindrops(27));
    }

    #[test]
    #[ignore]
    fn test_35() {
        assert_eq!("PlangPlong", raindrops(35));
    }

    #[test]
    #[ignore]
    fn test_49() {
        assert_eq!("Plong", raindrops(49));
    }

    #[test]
    #[ignore]
    fn test_52() {
        assert_eq!("52", raindrops(52));
    }

    #[test]
    #[ignore]
    fn test_105() {
        assert_eq!("PlingPlangPlong", raindrops(105));
    }

    #[test]
    #[ignore]
    fn test_3125() {
        assert_eq!("Plang", raindrops(3125));
    }

    #[test]
    #[ignore]
    fn test_12121() {
        assert_eq!("12121", raindrops(12_121));
    }
}

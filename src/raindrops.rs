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
/// assert_eq!("Pling", raindrops::raindrops(3));
/// assert_eq!("PlingPlang", raindrops::raindrops(15));
/// assert_eq!("52", raindrops::raindrops(52));
/// assert_eq!("PlingPlangPlong", raindrops::raindrops(105));
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

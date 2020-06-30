/// Given a number n, determine what the nth prime is.
/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
/// we can see that the 6th prime is 13.
///
/// # Parameters
///
/// - `n`: An u32 number
///
/// # Returns
///
/// - `ret_nth`: An u32 n-th prime number
///
/// # Examples
///
/// ```rust
/// assert_eq!(np::nth(1), 3);
/// assert_eq!(np::nth(10_000), 104_743);
/// ```
pub fn nth(n: u32) -> u32 {
    let mut ret_nth: u32;
    let mut prime_numbers: Vec<u32> = vec![];

    let mut counter: u32 = 0;

    if n == 0 {
        return 2;
    }

    loop {
        // Dangerous thing!
        ret_nth = counter;
        if is_prime(counter) {
            prime_numbers.push(counter);
            // -1 is used to ensure positioning
            // function input n considers that list starts from 0
            if (prime_numbers.len() - 1) as u32 == n {
                break;
            }
        }

        // Debug, just in case
        /*if counter == 100000 {
            break;
        }*/

        counter += 1;
    }

    /// Given a number n, determine is it a prime or not.
    ///
    /// # Parameters
    ///
    /// - `n`: An u32 number
    ///
    /// # Returns
    ///
    /// - `ret`: An `bool` -> true for prime; false for not a prime number
    ///
    /// # Examples
    ///
    /// ```rust
    /// println!("Result for {} is {}", 0, is_prime(0)); //false
    /// println!("Result for {} is {}", 1, is_prime(1)); //false
    /// println!("Result for {} is {}", 2, is_prime(2)); //true
    /// println!("Result for {} is {}", 3, is_prime(3)); //true
    /// println!("Result for {} is {}", 4, is_prime(4)); //false
    /// println!("Result for {} is {}", 5, is_prime(5)); //true
    /// println!("Result for {} is {}", 10, is_prime(10)); //false
    /// println!("Result for {} is {}", 15, is_prime(15)); //false
    /// println!("Result for {} is {}", 18, is_prime(18)); //false
    /// println!("Result for {} is {}", 19, is_prime(19)); //true
    /// ```
    fn is_prime(number: u32) -> bool {
        let mut ret: bool = true;
        let mut count: u32 = 2;

        // Zero and 1 are not considered prime numbers.
        if number < 2 {
            return false;
        }

        // The only even prime number is 2
        if number == 2 {
            return true;
        }

        // No prime number greater than 5 ends in a 5.
        if (number > 5) && (number % 5 == 0) {
            return false;
        }

        // If the sum of a number's digits is a multiple of 3,
        // that number can be divided by 3.
        if (number != 3) && (sum_digits(number) % 3 == 0) {
            return false;
        }

        while count < number {
            if number % count == 0 {
                ret = false;
                break;
            }
            count += 1;
        }

        /// Given a number n, determine is it a prime or not.
        ///
        /// # Parameters
        ///
        /// - `n`: An u32 number
        ///
        /// # Returns
        ///
        /// - `sum`: An u32 sum of the all digits in the input number
        ///
        /// # Examples
        ///
        /// ```rust
        /// println!("Result for {} is {}", 123, sum_digits(123));
        /// println!("Result for {} is {}", 5003, sum_digits(5003));
        /// ```
        fn sum_digits(n: u32) -> u32 {
            let mut sum: u32 = 0;
            let mut num = n;

            while num != 0 {
                sum += (num % 10) as u32;
                num = (num / 10) as u32;
            }
            sum // sum_digits return
        }
        ret // is_prime method return
    }
    ret_nth // nth method return
}

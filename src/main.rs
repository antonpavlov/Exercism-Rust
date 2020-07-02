mod helloworld;
mod leap;
mod nthprime;
mod raindrops;

fn main() {
    println!("Exercises:");
    println!("Hello World: {}", helloworld::hello());
    println!("Leap year: Is 2020 a leap year? - {}", leap::is_leap_year(2020));
    println!("Nth prime number: 19th prime number is {}", nthprime::nth(19));
    println!("Raindrops: 15 - {}", raindrops::raindrops(15));
}

//Project Euler Question 1 -- Sum of all numbers below UPPER_BOUND that are divisible by 3 and/or 5

const UPPER_BOUND: u32 = 10;

fn main() {
    let divisor1 = 3;
    let divisor2 = 5;
    let mut sum: u32 = 0;

    for n in 1..UPPER_BOUND {
        if n % divisor1 == 0 {
            sum += n;
        } else if n % divisor2 == 0 {
            sum += n;
        }
    }

    println!("The sum of all the multiples of {} or {} below {} is {}", divisor1, divisor2, UPPER_BOUND, sum);
}

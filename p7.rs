/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/

mod prime; // import prime.rs

fn main()
{
    static NUM_PRIMES: int = 10001;
    let mut prime_count = 0;
    let mut num: i64 = 2;
    while prime_count < NUM_PRIMES {
        if prime::is_prime(num) {
            prime_count += 1;
        }
        num += 1;
    }
    println!("{:d}", num-1);

}


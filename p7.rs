/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/

// long prime number test
fn long_test(num: i64) -> bool
{
    if 0 == num%2 || 0 == num%3 {
        return false;
    }
    for i in range(5 as i64, ((num as f64).sqrt() as i64) + 1)
    {
        if 0 == num%i {
            return false;
        }
    }
    return true;
}

// complete prime number test
fn is_prime(num: i64) -> bool
{
    match num {
        1 => false,
        0 => false,
        2 => true,
        3 => true,
        _ => long_test(num)
    }

}

fn main()
{
    static NUM_PRIMES: int = 10001;
    let mut prime_count = 0;
    let mut num: i64 = 2;
    while prime_count < NUM_PRIMES {
        if is_prime(num) {
            prime_count += 1;
        }
        num += 1;
    }
    println!("{:d}", num-1);

}


/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/

mod prime;

fn main()
{
    static MAX_PRIME: i64 = 2000000;
    let mut sum: i64 = 0;
    let mut num: i64 = 2;

    while num < MAX_PRIME {
        if prime::is_prime(num) {
            sum += num;
        }
        num += 1;
    }

    println!("{:d}", sum);
}

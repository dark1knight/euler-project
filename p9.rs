/*
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a2 + b2 = c2

For example, 32 + 42 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

extern crate std;
use std::num;

fn find_c(a: int, b: int) -> f64
{
    ((num::pow(a, 2) + num::pow(b, 2)) as f64).sqrt() 
}

fn main()
{
    let find_b = |a| (5*num::pow(10, 5) - 1000*a) as f64 / (1000-a) as f64;

    for a in range(1, 1000) {
        let b = find_b(a);
        if b.floor() != b || a > (b as int) {
            continue;
        }
        let c = find_c(a, b as int);
        if c.floor() == c {
            let p = a * (b as int) * (c as int);
            println!("{:d}, {:d}, {:d}, {}", a as int, b as int, c as int, p);
            break;
        }



    }
}

/*
A number chain is created by continuously adding the square of the digits in a number to form a new number until it has been seen before.

For example,

44 → 32 → 13 → 10 → 1 → 1
85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89

Therefore any chain that arrives at 1 or 89 will become stuck in an endless loop. What is most amazing is that EVERY starting number will eventually arrive at 1 or 89.

How many starting numbers below ten million will arrive at 89?
*/

extern crate std;
use std::num;

fn digit_squares(mut num: i64) -> bool
{
    match num {
        89 => return true,
        1 => return false,
        _ => () // let it fall through
    }

    // continue doing stuff to this number
    static TEN: i64 = 10;
    let mut new_num = 0;
    while num > 0 {
        new_num += num::pow(num%TEN, 2);
        num = num/TEN as i64;
    }
    return digit_squares(new_num);
}

fn main()
{
    let mut i: int = 10000000;
    let mut total: int = 0;

    while i > 0
    {
        match digit_squares(i as i64) {
            true => total += 1,
            false => ()
        }
        i -= 1;
    }
    println!("{:d}", total);
}

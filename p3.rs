/*
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600,851,475,143 ?
*/

mod prime;

fn main()
{
    // this number is too big for 32 bit ints
    static NUMBER: i64 = 600851475143;
    
    let mut num: i64 = (NUMBER as f64).sqrt() as i64 + 1;

    while num > 2 {
        if prime::is_prime(num) && NUMBER%num == 0 {
            break;
        }

        num -= 1;
    }
    println!("{:d}", num);

}

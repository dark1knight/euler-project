
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
pub fn is_prime(num: i64) -> bool
{
    match num {
        1 => false,
        0 => false,
        2 => true,
        3 => true,
        _ => long_test(num)
    }

}


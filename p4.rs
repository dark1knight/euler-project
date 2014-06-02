/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

fn is_palindrome(num: int) -> bool
{
	// convert the number to string
	let s: String = format!("{:d}", num);
	let s_view: &str = s.as_slice();
	let len = s_view.len();
	for i in range(0, len) {
		if s_view[i] != s_view[len-i-1]
		{
			return false;
		}
		match len-i-1-i {
			0 => return true,
			1 => return s_view[i] == s_view[i+1],
			_ => ()
		}
	}
	return true;
}

fn main()
{
	let mut pal: int = 0;
	let mut i = 999;
	while i > 99 && !done {
		let mut j = 999;
		while j > 99 && !done {
			if is_palindrome(i*j) && i*j > pal {
				pal = i*j;
			}
			j -= 1;
		}
		i -= 1;
	}
	println!("Biggest palindrome={:d}", pal);
}


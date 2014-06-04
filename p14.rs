/*
Longest Collatz sequence

The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?
*/

fn collatz(num: i64) -> uint
{
	match num {
		1 => return 1,
		_ => {
			match num%2 {
				0 => return collatz(num/2) + 1,
				1 => return collatz(3*num + 1) + 1,
				_ => 0
			}
		}
	}
}

fn main()
{
	let mut i: i64 = 1000000;
	let mut longest: i64 = 0;
	let mut biggest_collatz: uint = 0;
	while i > 0 {
		let c = collatz(i);
		if c > biggest_collatz {
			longest = i;
			biggest_collatz = c;
		}
		i -= 1;
	}
	println!("Number {:d} produces the biggest collatz sequence of {:u}", longest, biggest_collatz);
}

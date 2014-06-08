/*
It was proposed by Christian Goldbach that every odd composite number can be written as the sum of a prime and twice a square.

9 = 7 + 2×12
15 = 7 + 2×22
21 = 3 + 2×32
25 = 7 + 2×32
27 = 19 + 2×22
33 = 31 + 2×12

It turns out that the conjecture was false.

What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?
*/
#include <prime.h>
#include <iostream>
#include <vector>
#include <set>
#include <cassert>
#include <algorithm>

using namespace std;
bool goldbach(
	const uint64_t num, 
	const vector<uint64_t> &primes,
	const set<uint64_t> &squares)
{
	// find index of the first square that is bigger than num
	auto found = squares.upper_bound(num);

	for (auto i1(primes.begin() ); i1 != primes.end(); ++i1)
	{
		for (auto i2(squares.begin() ); i2 != found; ++i2)
		{
			if (*i1 + 2*(*i2) == num) {
				return true;
			}
		}
	}
	std::cout << num << std::endl;
	return false;
}

void compute_squares(
	set<uint64_t> &squares,
	const uint64_t how_many,
	const uint64_t starting_num)
{
	assert(how_many >= 0);
	for (uint64_t i = starting_num; i < starting_num+how_many; ++i)
	{
		squares.insert(i*i);
	}
}

int main()
{
	vector<uint64_t> primes = { 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31 };
	set<uint64_t> squares;
	compute_squares(squares, 100, 1);
	uint64_t last_square(100);
	enum { 
		SQUARE_STEP = 10,
   	    START_NUM = 33
	};

	for (uint64_t i = START_NUM; true; i+=2)
	{
		if (prime::is_prime(i) ) { // do nothing for primes
			primes.push_back(i);
			continue;
		}
		if (i > *(--squares.end() )/2 ) { // need to compute more squares
			compute_squares(squares, SQUARE_STEP, last_square);
			last_square += SQUARE_STEP;
		}

		// not prime - test goldbach
		if (!goldbach(i, primes, squares) ) {
			break;
		}
	}
	return 0;
}

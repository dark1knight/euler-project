#include <prime.h>
#include <cmath>

namespace {
bool long_test(const uint64_t num)
{
	if (0 == num%2 || 0 == num%3) {
		return false;
	}
	for (int i = 5; i < (uint64_t)std::sqrt(num) + 1; ++i)
	{
		if (0 == num%i) {
			return false;
		}
	}
	return true;
}
}

bool prime::is_prime(const uint64_t num)
{
	switch (num) {
		case 0:
		case 1:
			return false;
		case 2:
		case 3:
			return true;
		default:
			return long_test(num);
	}
	return false;
}

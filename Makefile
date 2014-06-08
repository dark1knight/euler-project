LINK=g++ -std=c++0x

p46: p46.cpp prime.cpp
	$(LINK) -I. p46.cpp prime.cpp -o p46


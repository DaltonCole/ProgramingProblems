from math import log10, floor

size = len(input())

n = 1
s = 0.0

while floor(s) < size:
	s += log10(n)
	n += 1

print(n-3)

"""
#include <iostream>
#include <math.h>
using namespace std;

int main() {
	string s;
	cin >> s;

	int fl = s.length();

	int n = 1;
	float sum = 0.0;

	while(floor(sum) < fl) {
		sum += log10(n);
		n++;
	}

	cout << n + 2 << endl;
}
"""
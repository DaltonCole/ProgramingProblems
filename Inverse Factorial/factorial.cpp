#include <iostream>
#include <math.h>
using namespace std;

// Works for big !

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
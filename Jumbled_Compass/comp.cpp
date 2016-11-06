#include <iostream>
#include <cmath>
using namespace std;

int main() {
	int a, b, c, d;

	cin >> a >> b;

	if(a > b) {
		c = 360 - a + b;
		d = -(a - b);
	} else {
		c = b - a;
		d = -(360 - b + a);
	}


	if(abs(c) <= abs(d)) {
		cout << c << endl;
	} else {
		cout << d << endl;
	}

	return 0;
}
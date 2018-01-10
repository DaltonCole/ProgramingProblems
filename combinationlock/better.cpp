#include <iostream>
#include <algorithm>
using namespace std;

int main() {
	int move = 360 / 40;
	int degrees = 0;

	int a, b, c, d;

	while(true) {
		cin >> a >> b >> c >> d;
		if(a == b && b == c && c == d && d == 0) {
			break;
		}
		degrees = 0;
		degrees += (360 * 3);
		if(b > a) {
			degrees -= (b - a) * move;
		} else {
			degrees -= (40 - a - b) * move;
		}
		if(c < b) {
			degrees -= (b - c) * move;
		} else {
			degrees -= (40 - c - b) * move;
		}
		if(d > c) {
			degrees -= (d - c) * move;
		} else {
			degrees -= (40 - c - d) * move;
		}

		cout << degrees << endl;
	}
	return 0;
}
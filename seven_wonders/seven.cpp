#include <iostream>
#include <cmath>
#include <algorithm>
using namespace std;

int main() {
	string input;
	int t = 0, c = 0, g = 0, answer = 0;

	cin >> input;

	for(auto it : input) {
		switch(it) {
			case 'T':
				t++;
				break;
			case 'C':
				c++;
				break;
			case 'G':
				g++;
				break;
			default:
				break;
		}
	}

	answer += pow(t,2) + pow(c, 2) + pow(g, 2) + (7 * min(t, min(c, g)));

	cout << answer << endl;

	return 0;
}
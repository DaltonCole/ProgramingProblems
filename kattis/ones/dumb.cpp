#include <iostream>

using namespace std;

int main() {
	unsigned long long number;
	unsigned long long one;
	int length;

	while(cin >> number) {
		one = 1;
		length = 1;

		while(one % number != 0) {
			one *= 10;
			one++;
			one %= number;
			length++;
		}

		cout << length << endl;
	}
}
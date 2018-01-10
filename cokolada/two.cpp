#include <iostream>
using namespace std;

int main() {
	int number;

	cin >> number;

	int two = 1;

	while(two < number) {
		two = two << 1;
	}
	cout << two << " ";

	int count = -1;

	while(number) {
		if(number >= two) {
			number -= two;
		}
		two = two >> 1;
		count++;
	}

	cout << count << endl;

	return 0;
}
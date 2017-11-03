#include <iostream>
using namespace std;

int main() {
	int value;
	int divisor = 2;
	int count = 0;

	cin >> value;

	while(value != 1) {
		if(value % divisor == 0) {
			cout << value << endl;
			value /= divisor;
			count++;
		} else {
			if (divisor == 2) {
				divisor += 1;
			} else {
				divisor += 2;
			}
		}
		//cout << value << endl;
	}
	cout << count << endl;

	return 0;
}
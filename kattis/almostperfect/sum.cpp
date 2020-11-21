#include <iostream>
#include <math.h>

using namespace std;

int main() {
	int number;
	int count;
	int square_root;

	while(cin >> number) {
		count = 1;
		square_root = int(sqrt(number));
		for(int i = 2; i <= square_root; i++) {
			if(number % i == 0) {
				count += (number / i);
				count += i;
			}
		}

		// Only count square_root once
		if(square_root * square_root == number) {
			count -= square_root;
		}

		if(count == number) {
			cout << number << " perfect" << endl;
		} else if(count - 2 == number || count + 2 == number || count - 1 == number || count + 1 == number) {
			cout << number << " almost perfect" << endl;
		} else {
			cout << number << " not perfect" << endl;
		}
	}
}
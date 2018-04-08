#include <iostream>
#include <algorithm>
#include <cstring>

using namespace std;

int main() {
	int cases;

	cin >> cases;

	int length;
	int odd_length;
	int even_length;
	int index;

	for(int i = 0; i < cases; i++) {
		cin >> length;

		if(length % 2 == 0) {
			odd_length = length / 2;
			even_length = odd_length;
		} else {
			odd_length = (length / 2) + 1;
			even_length = length / 2;
		}

		int* buffer_even = new int[even_length];
		int* buffer_odd = new int[odd_length];

		int odd = 0;
		int even = 0;
		for(int j = 0; j < length; j++) {
			if(j % 2 == 0) {
				cin >> buffer_odd[odd];
				odd++;
			} else {
				cin >> buffer_even[even];
				even++;
			}
		}

		sort(buffer_even, buffer_even + even);
		sort(buffer_odd, buffer_odd + odd);

		odd = 0;
		even = 0;
		index = -1;
		for(int j = 0; j < length; j++) {
			if(j % 2 == 0) {
				if(buffer_odd[odd] > buffer_even[even] && even < even_length) {
					index = j; break;
				}
				odd++;
			} else {
				if(buffer_even[even] > buffer_odd[odd]) {
					index = j; break;
				}
				even++;
			}
		}

		cout << "Case #" << (i + 1) << ": ";
		if(index == -1) {
			cout << "OK" << endl;
		} else {
			cout << index << endl;
		}

		delete[]  buffer_even;
		delete[] buffer_odd;
	}

	return 0;
}
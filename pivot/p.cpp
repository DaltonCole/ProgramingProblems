#include <iostream>

using namespace std;

#define N 100000

int smaller[N];
int larger[N];
int container[N];

int main() {
	int number;
	int temp;

	cin >> number;

	for(int i = 0; i < number; i++) {
		cin >> temp;
		container[i] = temp;
	}

	temp = 0;
	for(int i = 0; i < number; i++) {
		temp = (temp > container[i]) ? temp : container[i];
		smaller[i] = temp;
	}
	temp = 999999;
	for(int i = number-1; i >= 0; i--) {
		temp = (temp < container[i]) ? temp : container[i];
		larger[i] = temp;
	}

	temp = 0;
	for(int i = 0; i < number; i++) {
		if(container[i] >= smaller[i] && container[i] <= larger[i]) {
			temp++;
		}
	}

	cout << temp << endl;

	return 0;
}
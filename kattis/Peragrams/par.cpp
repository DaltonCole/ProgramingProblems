#include <iostream>
using namespace std;

int main() {
	string input;
	int arr[26];

	int remove = 0;

	cin >> input;

	for(int i = 0; i < 26; i++) {
		arr[i] = 0;
	}

	for(int i = 0; i < input.size(); i++) {
		arr[input[i] - 97]++;
	}


	for(int i = 0; i < 26; i++) {
		if(arr[i] % 2 != 0) {
			remove++;
		}
	}

	if(remove != 0) {
		remove--;
	}

	cout << remove << endl;

	return 0;
}
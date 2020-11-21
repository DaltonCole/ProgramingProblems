#include <iostream>

using namespace std;

bool they_do_overlap(const string& a, const string& b, const int& k) {

	for(int i = 0; i <= k; i++) {
		if(b[i] != a[i + (a.length() - k - 1)]) {
			return false;
		}
	}
	return true;
}

int main() {
	int cases;
	int length = 0;
	int words = 0;
	int letters = 0;
	int overlap;

	string a;
	string b;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> length >> words;

		letters = length;
		cin >> a;
		for(int j = 1; j < words; j++) {
			cin >> b;
			overlap = 0;

			for(int k = 0; k < length; k++) {
				if(they_do_overlap(a, b, k)) {
					overlap = k + 1;
				}
			}

			letters += (length - overlap);

			a = b;
		}

		cout << letters << endl;
	}

	return 0;
} 
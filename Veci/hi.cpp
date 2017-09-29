#include <iostream>
#include <algorithm>

using namespace std;

int main() {
	string word;

	cin >> word;

	auto a = next_permutation(word.begin(), word.end());

	if(a) {
		cout << word << endl;
	} else {
		cout << 0 << endl;
	}
}
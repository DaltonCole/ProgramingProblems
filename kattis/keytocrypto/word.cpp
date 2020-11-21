#include <iostream>

using namespace std;

int main() {
	string cry;
	string key;
	int letter;

	cin >> cry;
	cin >> key;

	int index = 0;

	for(int i = 0; i < cry.length(); i++) {
		letter = (cry[i] - (key[index] - 'A'));
		//cout << letter << endl;
		if(letter < 65) {
			letter += 26;
		}
		cout << char(letter);
		index = (index + 1) % key.length();
	}
	cout << endl;

	return 0;
}
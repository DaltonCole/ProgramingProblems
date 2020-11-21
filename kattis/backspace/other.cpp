#include <iostream>
#include <string>
#include <algorithm>

using namespace std;

int main() {
	string word;
	string new_word = "";
	int back = 0;

	cin >> word;

	for(int i = word.length() - 1; i >= 0; i--) {
		if(word[i] == '<') {
			back++;
		} else if(back != 0) {
			back--;
		} else {
			new_word += word[i];
		}
	}

	for(int i = new_word.length() - 1; i >= 0; i--) {
		cout << new_word[i];
	}
	cout << endl;


	return 0;
}
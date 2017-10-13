#include <iostream>
#include <unordered_map>

using namespace std;

int main () {
	int inputs;
	string word, sub_word;

	unordered_map<string, int> count;

	cin >> inputs;

	for(int i = 0; i < inputs; i++) {
		cin >> word;

		cout << count[word] << endl;

		sub_word = "";
		for(int j = 0; j < word.length(); j++) {
			sub_word += word[j];
			count[sub_word]++;
		}
	}

	return 0;
}
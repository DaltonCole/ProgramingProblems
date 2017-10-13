#include <iostream>
#include <unordered_set>
#include <string>

using namespace std;

int main() {
	string word, sub_word;
	unordered_set<string> dict;

	unordered_set<string>::const_iterator got;

	int shortest = 999999, longest = 0;
	int total;

	while(cin >> word) {
		if(word == "#") {
			break;
		}
		dict.insert(word);

		if(word.length() > longest) {
			longest = word.length();
		}
		if(word.length() < shortest) {
			shortest = word.length();
		}
	}

	while(cin >> word) {
		if(word == "#") {
			break;
		}
		total = 0;

		for(int i = 0; i < (word.length() - shortest - 1); i++) {
			sub_word = "";

			// Make sub-word of at least shortest length
			for(int j = 0; j < shortest; j++) {
				sub_word += word[i + j];
			}

			got = dict.find(sub_word);

			if(got != dict.end()) {
				cout << *got << endl;
				total++;
			}

			for(int j = i + shortest; j < (word.length() - 1) && sub_word.length() <= longest; j++) {
				sub_word += word[j];
				got = dict.find(sub_word);
				if(got != dict.end()) {
					cout << *got << endl;
					total++;
				}
			}
		}
		cout << total << endl;
	}
}
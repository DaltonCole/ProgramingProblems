#include <iostream>
#include <string>
#include <unordered_map>

using namespace std;

int main() {
	unordered_map<string, int> words;
	string word, sub_word, smaller_word;
	int length;

	cin >> length;

	cin >> word;

	for(int i = length - 1; i > 0; i--) {
		words.clear();
		for(int j = 0; j < (length - i); j++) {
			sub_word = word.substr(j, i);
			words[sub_word]++;
			if(words[sub_word] > 1) {
				cout << i << endl;
				return 0;
			}
		}
		//cout << i << endl;
	}

	cout << 0 << endl;
	return 0;
}
/*
	for(int i = 0; i < length; i++) {
		sub_word = "";
		sub_word += word[i];
		words[sub_word]++;

		for(int j = i + 1; j < length; j++) {
			sub_word += word[j];
			words[sub_word]++;
		}
	}

	length = 0;
	for(auto w : words) {
		if(w.second > 1) {
			if(w.first.length() > length) {
				length = w.first.length();
			}
		}
	}

	cout << length << endl;
}
*/
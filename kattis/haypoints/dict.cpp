#include <iostream>
#include <unordered_map>
#include <string>

using namespace std;

int main() {
	int number_of_dict_words;
	int descriptions;

	string word;
	int value;

	int total;

	unordered_map<string, int> map;

	cin >> number_of_dict_words >> descriptions;

	for(int i = 0; i < number_of_dict_words; i++) {
		cin >> word >> value;

		map[word] = value;
	}

	for(int i = 0; i < descriptions; i++) {
		total = 0;

		while(true) {
			cin >> word;

			if(word == ".") {
				break;
			}

			total += map[word];
		}

		cout << total << endl;
	}
}
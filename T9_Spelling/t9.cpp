#include <iostream>
#include <unordered_map>
#include <string>
using namespace std;

int main() {
	unordered_map<string, string> map;
	int cases = 0;
	string word;
	string t9;

	map["a"] = "2";
	map["b"] = "22";
	map["c"] = "222";
	map["d"] = "3";
	map["e"] = "33";
	map["f"] = "333";
	map["g"] = "4";
	map["h"] = "44";
	map["i"] = "444";
	map["j"] = "5";
	map["k"] = "55";
	map["l"] = "555";
	map["m"] = "6";
	map["n"] = "66";
	map["o"] = "666";
	map["p"] = "7";
	map["q"] = "77";
	map["r"] = "777";
	map["s"] = "7777";
	map["t"] = "8";
	map["u"] = "88";
	map["v"] = "888";
	map["w"] = "9";
	map["x"] = "99";
	map["y"] = "999";
	map["z"] = "9999";
	map[" "] = "0";

	cin >> cases;
	getline(cin, word);

	for(int i = 0; i < cases; i++) {
		getline(cin, word);

		t9 += map[string(1, word[0])];

		for(int j = 1; j < word.length(); j++) {
			if((t9[t9.length() - 1]) == (map[string(1, word[j])][0])) {
				t9 += " ";
			}
			t9 += map[string(1, word[j])];
		}

		cout << "Case #" << i+1 << ": " << t9 << endl;
		t9 = "";
	}

	return 0;
}
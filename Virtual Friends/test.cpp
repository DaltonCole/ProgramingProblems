#include <iostream>
#include <unordered_map>
#include <unordered_set>

using namespace std;

int main() {
	int cases;

	int friendships;

	string a, b;

	unordered_set<int> * ham = new unordered_set<int>;

	cases = 5;

	ham -> insert(cases);

	unordered_map<string, unordered_set<string>* > test;

	a = "test";

	cout << "b" << endl;

	if(test["a"] == nullptr) {
		test["a"] = new unordered_set<string>;
	}

	test["a"] -> insert(a);

	cout << "c" << endl;

	cout << test["a"] -> size() << endl;


	/*
	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> friendships;

		for(int j = 0; j < friendships; j++) {
			cin >> a >> b;

			friends[a] -> insert(a);
			friends[a].insert(b);

			friends[a].insert(friends[b].begin(), friends[b].end());
			friends[b] = friends[a];

			cout << friends[a].size() << endl;
		}

	}
	*/

	return 0;
}
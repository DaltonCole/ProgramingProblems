#include <iostream>
#include <vector>
#include <algorithm>
#include <unordered_map>

using namespace std;

int main() {
	int lines = 0;
	unordered_map<string, vector<int> > timeline;
	string name;
	int year;

	cin >> lines;

	for(int i = 0; i < lines; i++) {
		cin >> name >> year;
		timeline[name].push_back(year);
	}

	for(auto& i : timeline) {
		sort(i.second.begin(), i.second.end());
	}

	cin >> lines;

	for(int i = 0; i < lines; i++) {
		cin >> name >> year;
		cout << timeline[name][year-1] << endl;
	}

	return 0;
}
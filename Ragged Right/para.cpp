#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
	int n;
	int m;
	int total = 0;
	string line;

	vector<int> line_chars;

	while(getline(cin, line)) {
		line_chars.push_back(line.length());
	}

	n = *max_element(line_chars.begin(), line_chars.end());

	for(int i = 0; i < line_chars.size() - 1; i++) {
		total += ((n - line_chars[i]) * (n - line_chars[i]));
	}

	cout << total << endl;

	return 0;
}
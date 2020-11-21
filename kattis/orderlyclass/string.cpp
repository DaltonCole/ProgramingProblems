#include <iostream>
#include <unordered_map>

using namespace std;

bool same_char(const string & a, const string & b) {
	unordered_map<char, int> m;

	for(int i = 0; i < a.length(); i++) {
		m[a[i]]++;
	}

	for(int i = 0; i < a.length(); i++) {
		m[b[i]]--;
	}

	for(auto i : m) {
		if(i.second != 0) {
			return false;
		}
	}
	return true;
}

int main() {
	string x, y;

	cin >> x >> y;

	if(same_char(x, y) == false) {
		cout << 0 << endl;
		return 0;
	}

	int start = - 1;
	int end = -1;

	for(int i = 0; i < x.length(); i++) {
		if(start == -1 && x[i] != y[i]) {
			start = i;
		} else if(x[i] != y[i]) {
			end = i;
		}
	}

	string sub_a = "";
	string sub_b = "";

	for(int i = end; i >= start; i--) {
		sub_a += x[i];
	}

	for(int i = start; i <= end; i++) {
		sub_b += y[i];
	}

	int count = 0;

	if(sub_a == sub_b) {
		count++;
	}

	while(count) {
		if(start <= 0 || end >= x.length() - 1) {
			break;
		}
		start--;
		end++;
		if(x[start] == x[end] && y[start] == y[end]) { // Don't need y
			count++;
		} else {
			break;
		}
	}

	cout << count << endl;

	return 0;
}
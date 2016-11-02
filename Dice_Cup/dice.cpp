#include <iostream>
#include <map>
using namespace std;

int main() {
	int a, b, max = 0, element = 0;

	map<int, int> count;

	cin >> a >> b;

	for(int i = 1; i <= a; i++) {
		for(int j = 1; j <= b; j++) {
			count[i+j] = count[i+j] + 1;
		}
	}

	for(auto & i : count) {
		if(i.second > max) {
			max = i.second;
			element = i.first;
		}
	}

	for(auto & i : count) {
		if(max == i.second) {
			cout << i.first << endl;
		}
	}

	return 0;
}
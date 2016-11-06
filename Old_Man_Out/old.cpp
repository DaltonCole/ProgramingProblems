#include <iostream>
#include <unordered_map>
using namespace std;

int main() {
	int cases = 0;
	int guests = 0;
	int guest_number = 0;
	unordered_map<long, int> map;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> guests;
		for(int j = 0; j < guests; j++) {
			cin >> guest_number;
			map[guest_number] += 1;
		}


		for(auto it : map) {
			if(it.second == 1) {
				cout << "Case #" << i+1 << ": " << it.first << endl;
			}
		}

		map.clear();
	}

	return 0;
}
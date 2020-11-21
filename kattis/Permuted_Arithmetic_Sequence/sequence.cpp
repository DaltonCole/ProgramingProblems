#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
	int cases;
	int numbers;
	int temp;
	vector<int> list;
	int diff1, diff2;
	bool worked = true;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> numbers;

		for(int j = 0; j < numbers; j++) {
			cin >> temp;
			list.push_back(temp);
		}

		diff1 = list[1] - list[0];

		for(int j = 2; j < numbers; j++) {
			if(diff1 != list[j] - list[j-1]) {
				worked = false;
				break;
			}
		}

		if(worked == true) {
			cout << "arithmetic" << endl;
		} else {

			worked = true;

			sort(list.begin(), list.end());

			diff1 = list[1] - list[0];

			for(int j = 2; j < numbers; j++) {
				if(diff1 != list[j] - list[j-1]) {
					worked = false;
					break;
				}
			}

			if(worked == true) {
				cout << "permuted arithmetic" << endl;
			} else {
				cout << "non-arithmetic" << endl;
			}
		}

		worked = true;
		list.clear();
	}
}
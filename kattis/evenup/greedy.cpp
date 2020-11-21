#include <iostream>
#include <vector>

using namespace std;

int main() {
	int cards, temp;
	vector<int> row;
	bool had_removal = false;

	cin >> cards;

	for(int i = 0; i < cards; i++) {
		cin >> temp;
		row.push_back(temp);
	}

	while(true) {
		for(int j = 0; j < row.size() - 1; j++) {
			if((row[j] + row[j+1]) % 2 == 0) {
				// Remove element j and j+1
				if(row.size() > 2) {
					row.erase(row.begin() + j, row.begin() + j + 2);
					had_removal = true;
				} else {
					cout << 0 << endl;
					return 0;
				}
			}
		}

		// Make sure we removed something, else we will never remove anymore
		if(had_removal == true) {
			had_removal = false;
		} else {
			break;
		}
	}

	cout << row.size() << endl;

	return 0;
}
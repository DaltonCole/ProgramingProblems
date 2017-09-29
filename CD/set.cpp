#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
	int jake_inputs;
	int jill_inputs;
	int temp;

	while(true) {
		vector<int> jake;
		vector<int> jill;
		
		cin >> jake_inputs >> jill_inputs;

		if(jake_inputs == 0 and jill_inputs == 0) {
			break;
		}

		for(int i = 0; i < jake_inputs; i++) {
			cin >> temp;
			jake.push_back(temp);
		}

		for(int i = 0; i < jill_inputs; i++) {
			cin >> temp;
			jill.push_back(temp);
		}

		int a = 0, b = 0, sell = 0;

		while(a < jake_inputs && b < jill_inputs) {
			if(jake[a] < jill[b]) {
				a++;
			} else if (jake[a] > jill[b]) {
				b++;
			} else {
				a++;
				b++;
				sell++;
			}
		}

		cout << sell << endl;
	}

	return 0;
}
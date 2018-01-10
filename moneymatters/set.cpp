#include <iostream>
#include <set>
#include <vector>

using namespace std;

// Put index in sets
// Put values in vector
// set index goes with vector value

int main() {
	int friends;
	int bonds;
	int temp;
	int a, b;

	cin >> friends >> bonds;

	vector<set<int> > price;
	vector<int> index;

	int indexa;
	int indexb;



	for(int i = 0; i < friends; i++) {
		cin >> temp;
		index.push_back(temp);
	}

	for(int j = 0; j < bonds; j++) {
		cin >> a >> b;

		indexa = -1;
		indexb = -1;

		// Find set a
		for(int k = 0; k < price.size(); k++) {
			auto it = price[k].find(a);
			if(it != price[k].end()) {
				indexa = k;
				break;
			}
		}
		// Find set b
		for(int k = 0; k < price.size(); k++) {
			auto it = price[k].find(b);
			if(it != price[k].end()) {
				indexb = k;
				break;
			}
		}

		// Neither are in a set
		if(indexa == -1 && indexb == -1) {
			// Make new set
			price.push_back(set<int>());
			price[price.size() - 1].insert(a);
			price[price.size() - 1].insert(b);
		} else if(indexa == -1) { // No index a
			price[indexb].insert(a);
		} else if(indexb == -1) { // No index b
			price[indexa].insert(b);
		} else { // Combind sets
			price[indexa].insert(price[indexb].begin(), price[indexb].end());
			price.erase(price.begin() + indexb);
		}
	}

	/*
	for(auto& a : price) {
		cout << "Set:" << endl;
		for(auto& b : a) {
			cout << b << ", ";
		}
		cout << endl;
	}
	*/

	int money = 0;
	bool bad = false;

	for(auto& m : price) {
		money = 0;
		for(auto& bill : m) {
			money += index[bill];
		}
		if(money != 0) {
			bad = true;
			break;
		}
	}

	if(bad) {
		cout << "IMPOSSIBLE" << endl;
	} else {
		cout << "POSSIBLE" << endl;
	}

	return 0;
}
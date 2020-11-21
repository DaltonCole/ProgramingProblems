#include <iostream>
#include <unordered_set>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
	vector<unordered_set<int> > set_list;
	//unordered_set<int>*  
	int temp, queries;
	char op;
	int index1, index2, num1, num2;

	cin >> temp >> queries;

	for(int i = 0; i < queries; i++) {
		cin >> op >> num1 >> num2;

		// Make edge case of same query faster
		if(num1 == num2) {
			cout << "yes" << endl;
			continue;
		}


		index1 = -1;
		index2 = -2;

		// Find index of first element
		for(int x = 0; x < set_list.size(); x++) {
			if(set_list[x].find(num1) != set_list[x].end()) {
				index1 = x;
				break;
			}
		}
		// Find index of second element
		for(int x = 0; x < set_list.size(); x++) {
			if(set_list[x].find(num2) != set_list[x].end()) {
				index2 = x;
				break;
			}
		}

		// Query
		if(op == '?') {
			

			if(index1 == index2) {
				cout << "yes" << endl;
			} else {
				cout << "no" << endl;
			}
		} else { // Union
			// They are not in a set yet.
			if(index1 == -1 && index2 == -2) {
				set_list.emplace_back();
				set_list[set_list.size() - 1].insert(num1);
				set_list[set_list.size() - 1].insert(num2);
			} else if(index2 == -2) { // Num1 exists but not Num2
				set_list[index1].insert(num2);
			} else if(index1 == -1) { // Num2 exists but not Num1
				set_list[index2].insert(num1);
			} else if(index1 == index2) { // They are in the same set
				// skip
			} else { // They are in different sets
				set_list[index1].insert(set_list[index2].begin(), set_list[index2].end());
				set_list.erase(set_list.begin() + index2);
			}
		}

		/*
		cout << "start" << endl;
		for(auto& q : set_list) {
			for(auto& p : q) {
				cout << p << " ";
			}
			cout << endl;
		}
		*/
	}

	return 0;
}

/*
  std::unordered_set<std::string> second ( {"red","green","blue"} );    // init list
    myvector.emplace_back (100);
*/

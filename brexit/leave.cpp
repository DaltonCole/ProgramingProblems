#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <algorithm>    // std::set_union, std::sort
#include <queue>

using namespace std;

int main() {
	int num_contires;
	int num_partnerships;
	int home;
	int leave;

	int tmp1, tmp2, top;

	cin >> num_contires >> num_partnerships >> home >> leave;

	if(home == leave) {cout << "leave" << endl; return 0;}

	unordered_map<int, unordered_set<int> > partner;
	unordered_map<int, unordered_set<int> > left;
	unordered_set<int> leaves;
	queue<int> look_at;

	look_at.push(leave);

	for(int i = 0; i < num_contires; i++) {
		partner[i] = unordered_set<int>();
		left[i] = unordered_set<int>();
	}

	for(int i = 0; i < num_partnerships; i++) {
		cin >> tmp1 >> tmp2;
		partner[tmp1].insert(tmp2);
		partner[tmp2].insert(tmp1);
	}

	while(look_at.empty() == false) {
		top = look_at.front();
		look_at.pop();

		// They already left
		if(leaves.find(top) != leaves.end()) {
			continue;
		} else {
			leaves.insert(top);
		}

		for(auto possible : partner[top]) {
			left[possible].insert(top);

			if(2*left[possible].size() >= partner[possible].size()) {
				if(possible == home) {
					cout << "leave" << endl; 
					return 0;
				}
				look_at.push(possible);
			}
		}

	}

	cout << "stay" << endl;

	return 0;
}
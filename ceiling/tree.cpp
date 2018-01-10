#include <iostream>
#include <unordered_set>
#include <vector>
#include <cmath>

using namespace std;

int main() {
	int cases, nodes, temp, index;
	unordered_set<vector<bool> > different;

	cin >> cases >> nodes;

	for(int i = 0; i < cases; i++) {
		vector<int> tree(pow(2, nodes), 0);
		vector<bool> bool_tree(pow(2, nodes), false);

		cin >> temp;
		tree[0] = temp;
		bool_tree[0] = true;

		for(int j = 1; j < nodes; j++) {
			cin >> temp;
			index = 0;
			while(true) {
				if(bool_tree[index] == false) {
					bool_tree[index] = true;
					tree[index] = temp;
					break;
				} else if(tree[index] > temp) {
					index = (2 * index) + 1;
				} else if(tree[index] < temp) {
					index = (index * 2) + 2;
				}
			}
		}
		different.insert(bool_tree);
	}

	cout << different.size() << endl;
}
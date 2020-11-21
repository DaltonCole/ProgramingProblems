#include <vector>
#include <iostream>
#include <utility>
#include <cmath>
#include <set>
#include <unordered_map>
#include <algorithm>
#include <unordered_set>

using namespace std;

int node_count;
int grid[21][21];

int shortest_distance(const vector<string> names) {
	int min_path = 2147483647;

	vector<int> indexes;

	for(int i = 1; i < names.size(); i++) {
		indexes.push_back(i);
	}

	do {
		int path_length = 0;

		unordered_set<string> found_names;

		int begining = 0;
		for(int i = 0; i < indexes.size(); i++) {
			if(found_names.find(names[i]) == found_names.end()) {
				path_length += grid[begining][indexes[i]];
				begining = indexes[i];
				found_names.insert(names[i]);
			}
		}
		path_length += grid[begining][0];

		min_path = min(min_path, path_length);
	}while(next_permutation(indexes.begin(), indexes.end()));

	return min_path;
}


int main() {
	vector<pair<int, int> > nodes(1, make_pair(0,0));
	vector<string> names(1, "*");
	int x, y;
	string name;

	// Get user information
	cin >> node_count;

	for(int i = 0; i < node_count; i++) {
		cin >> x >> y >> name;
		nodes.push_back(make_pair(x, y));
		names.push_back(name);
	}
	node_count++;

	// Find distances and populate grid
	for(int i = 0; i < node_count; i++) {
		for(int j = 0; j < node_count; j++) {
			grid[i][j] = abs(nodes[i].first - nodes[j].first) + abs(nodes[i].second - nodes[j].second);
		}
	}

	/*
	for(int i = 0; i < node_count; i++) {
		for(int j = 0; j < node_count; j++) {
			cout << grid[i][j] << "\t";
		}
		cout << endl;
	}
	*/

	cout << shortest_distance(names) << endl;

	return 0;
}
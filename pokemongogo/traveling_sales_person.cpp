#include <vector>
#include <iostream>
#include <utility>
#include <cmath>
#include <set>
#include <unordered_map>
#include <algorithm>

using namespace std;

int node_count;
int grid[21][21];

void make_node_combinations_recursiong(const set<int>& indexes, vector<vector<int> >& solo_combinations) {
	if(indexes.size() == 1) {
		for(const auto& index : indexes) {
			for(int i = 0; i < solo_combinations.size(); i++) {
				solo_combinations[i].push_back(index);
			}
		}
	} else {
		auto original = solo_combinations;
		solo_combinations = vector<vector<int> >();
		for(const auto& index : indexes) {
			auto copy = original;
			for(int i = 0; i < copy.size(); i++) {
				copy[i].push_back(index);
			}
			for(auto& c : copy) {
				solo_combinations.push_back(c);
			}
		}
	}
}

vector<vector<int> > make_node_combinations(const vector<string> names) {
	unordered_map<string, set<int> > name_to_indexes;

	for(int i = 0; i < names.size(); i++) {
		name_to_indexes[names[i]].insert(i + 1);
	}

	vector<vector<int> > solo_combinations = {{}};
	for(const auto& same_pokemon : name_to_indexes) {
		make_node_combinations_recursiong(same_pokemon.second, solo_combinations);
	}

	for(auto& combo : solo_combinations) {
		sort(combo.begin(), combo.end());
	}

	return solo_combinations;
}

int shortest_distance(vector<int>& indexes) {
	int min_path = 2147483647;
	do {
		int path_length = 0;

		int begining = 0;
		for(int i = 0; i < indexes.size(); i++) {
			path_length += grid[begining][indexes[i]];
			begining = indexes[i];
		}
		path_length += grid[begining][0];

		min_path = min(min_path, path_length);
	}while(next_permutation(indexes.begin(), indexes.end()));

	return min_path;
}


int main() {
	vector<pair<int, int> > nodes(1, make_pair(0,0));
	vector<string> names;
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

	auto all_possible_node_combos = make_node_combinations(names);
	int path_length = 2147483647;

	for(auto& node_combo : all_possible_node_combos) {
		path_length = min(path_length, shortest_distance(node_combo));
	}

	cout << path_length << endl;

	return 0;
}
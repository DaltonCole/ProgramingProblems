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
vector<unordered_set<int> > closest_nodes;
vector<unordered_set<string> > node_names;
vector<pair<int, int> > nodes(1, make_pair(0,0));
vector<string> names(1, "*");
unordered_set<string> all_names;

// seen that = names
int min_distance(unordered_set<string> seen_that, int distance, int node_to_add) {
	// Add pokemon at node
	for(auto& n : node_names[node_to_add]) {
		seen_that.insert(n);
	}
	
	/*
	cout << "------" << endl;
	cout << seen_that.size() << endl;
	for(auto& n : seen_that) {
		cout << n << " ";
	}
	cout << endl;
	*/

	// See if we have all of the pokemon
	if(seen_that.size() == all_names.size()) {
		// If so, return to beginning
		//cout << node_to_add << " " << (distance + grid[0][node_to_add]) << endl;
		return distance + grid[0][node_to_add];
	}

	int best_distance = 2147483647;
	for(const auto& node : closest_nodes[node_to_add]) {
		if(seen_that.find(names[node]) == seen_that.end()) {
			best_distance = min(best_distance, min_distance(seen_that, distance + grid[node_to_add][node], node));
		}
	}
	return best_distance;
}

void populate_closest_nodes() {
	pair<int, int> closest_node; // distance, index
	vector<string> temp_size(21, "");

	for(int i = 0; i < nodes.size(); i++) {
		unordered_set<int> neighbors;

		int tries = 0;
		do{
			tries++;
		closest_node.first = 2147483647;
		// Closest north node
		for(int j = 1; j < nodes.size(); j++) {
			if(neighbors.find(j) != neighbors.end()) {
				continue;
			}

			// If neighbor node is same type(s) as this node, don't look at it
			// Check if j is a subset of i's names, if so, skip it
			if(set_difference(node_names[j].begin(), node_names[j].end(),
				node_names[i].begin(), node_names[i].end(), temp_size.begin()) - temp_size.begin() == 0) {
				continue;
			}

			if(nodes[i].first < nodes[j].first) {
				if(grid[i][j] < closest_node.first) {
					closest_node.first = grid[i][j];
					closest_node.second = j;
				}
			}
		}
		if(closest_node.first != 2147483647) {
			neighbors.insert(closest_node.second);
			closest_node.first = 2147483647;
		}
		// Closest south node
		for(int j = 1; j < nodes.size(); j++) {
			if(neighbors.find(j) != neighbors.end()) {
				continue;
			}

			// If neighbor node is same type(s) as this node, don't look at it
			// Check if j is a subset of i's names, if so, skip it
			if(set_difference(node_names[j].begin(), node_names[j].end(),
				node_names[i].begin(), node_names[i].end(), temp_size.begin()) - temp_size.begin() == 0) {
				continue;
			}

			if(nodes[i].first > nodes[j].first) {
				if(grid[i][j] < closest_node.first) {
					closest_node.first = grid[i][j];
					closest_node.second = j;
				}
			}
		}
		if(closest_node.first != 2147483647) {
			neighbors.insert(closest_node.second);
			closest_node.first = 2147483647;
		}
		// Closest east node
		for(int j = 1; j < nodes.size(); j++) {
			if(neighbors.find(j) != neighbors.end()) {
				continue;
			}

			// If neighbor node is same type(s) as this node, don't look at it
			// Check if j is a subset of i's names, if so, skip it
			if(set_difference(node_names[j].begin(), node_names[j].end(),
				node_names[i].begin(), node_names[i].end(), temp_size.begin()) - temp_size.begin() == 0) {
				continue;
			}

			if(nodes[i].second < nodes[j].second) {
				if(grid[i][j] < closest_node.first) {
					closest_node.first = grid[i][j];
					closest_node.second = j;
				}
			}
		}
		if(closest_node.first != 2147483647) {
			neighbors.insert(closest_node.second);
			closest_node.first = 2147483647;
		}
		// Closest west node
		for(int j = 1; j < nodes.size(); j++) {
			if(neighbors.find(j) != neighbors.end()) {
				continue;
			}
			
			// If neighbor node is same type(s) as this node, don't look at it
			// Check if j is a subset of i's names, if so, skip it
			if(set_difference(node_names[j].begin(), node_names[j].end(),
				node_names[i].begin(), node_names[i].end(), temp_size.begin()) - temp_size.begin() == 0) {
				continue;
			}

			if(nodes[i].second > nodes[j].second) {
				if(grid[i][j] < closest_node.first) {
					closest_node.first = grid[i][j];
					closest_node.second = j;
				}
			}
		}
		if(closest_node.first != 2147483647) {
			neighbors.insert(closest_node.second);
			closest_node.first = 2147483647;
		}

		}while(tries < 10 && closest_nodes.size() < 4);




		/*

		closest_node.first = 2147483647;
		// Closest north node
		for(int j = 1; j < nodes.size(); j++) {
			// If neighbor node is same type(s) as this node, don't look at it
			// Check if j is a subset of i's names, if so, skip it
			if(set_difference(node_names[j].begin(), node_names[j].end(),
				node_names[i].begin(), node_names[i].end(), temp_size.begin()) - temp_size.begin() == 0) {
				continue;
			}

			if(nodes[i].first < nodes[j].first && nodes[i].second < nodes[j].second) {
				if(grid[i][j] < closest_node.first) {
					closest_node.first = grid[i][j];
					closest_node.second = j;
				}
			}
		}
		if(closest_node.first != 2147483647) {
			neighbors.insert(closest_node.second);
			closest_node.first = 2147483647;
		}
		// Closest south node
		for(int j = 1; j < nodes.size(); j++) {
			// If neighbor node is same type(s) as this node, don't look at it
			// Check if j is a subset of i's names, if so, skip it
			if(set_difference(node_names[j].begin(), node_names[j].end(),
				node_names[i].begin(), node_names[i].end(), temp_size.begin()) - temp_size.begin() == 0) {
				continue;
			}

			if(nodes[i].first < nodes[j].first && nodes[i].second > nodes[j].second) {
				if(grid[i][j] < closest_node.first) {
					closest_node.first = grid[i][j];
					closest_node.second = j;
				}
			}
		}
		if(closest_node.first != 2147483647) {
			neighbors.insert(closest_node.second);
			closest_node.first = 2147483647;
		}
		// Closest east node
		for(int j = 1; j < nodes.size(); j++) {
			// If neighbor node is same type(s) as this node, don't look at it
			// Check if j is a subset of i's names, if so, skip it
			if(set_difference(node_names[j].begin(), node_names[j].end(),
				node_names[i].begin(), node_names[i].end(), temp_size.begin()) - temp_size.begin() == 0) {
				continue;
			}

			if(nodes[i].first > nodes[j].first && nodes[i].second < nodes[j].second) {
				if(grid[i][j] < closest_node.first) {
					closest_node.first = grid[i][j];
					closest_node.second = j;
				}
			}
		}
		if(closest_node.first != 2147483647) {
			neighbors.insert(closest_node.second);
			closest_node.first = 2147483647;
		}
		// Closest west node
		for(int j = 1; j < nodes.size(); j++) {
			// If neighbor node is same type(s) as this node, don't look at it
			// Check if j is a subset of i's names, if so, skip it
			if(set_difference(node_names[j].begin(), node_names[j].end(),
				node_names[i].begin(), node_names[i].end(), temp_size.begin()) - temp_size.begin() == 0) {
				continue;
			}

			if(nodes[i].first > nodes[j].first && nodes[i].second > nodes[j].second) {
				if(grid[i][j] < closest_node.first) {
					closest_node.first = grid[i][j];
					closest_node.second = j;
				}
			}
		}
		if(closest_node.first != 2147483647) {
			neighbors.insert(closest_node.second);
			closest_node.first = 2147483647;
		}
		*/
		closest_nodes.push_back(neighbors);
	}
}

int main() {
	int x, y;
	string name;

	// Get user information
	cin >> node_count;

	all_names.insert("*");

	for(int i = 0; i < node_count; i++) {
		cin >> x >> y >> name;
		nodes.push_back(make_pair(x, y));
		names.push_back(name);
		all_names.insert(name);
		node_names.push_back(unordered_set<string>());
	}
	node_count++;
	node_names.push_back(unordered_set<string>());

	// Find distances and populate grid
	for(int i = 0; i < node_count; i++) {
		for(int j = 0; j < node_count; j++) {
			grid[i][j] = abs(nodes[i].first - nodes[j].first) + abs(nodes[i].second - nodes[j].second);
			if(grid[i][j] == 0) {
				node_names[i].insert(names[j]);
			}
		}
	}

	///*
	for(int i = 0; i < node_names.size(); i++) {
		cout << i << ": ";
		for(auto j : node_names[i]) {
			cout << j << " ";
		}
		cout << endl;
	}
	cout << endl;

	
	for(int i = 0; i < node_count; i++) {
		for(int j = 0; j < node_count; j++) {
			cout << grid[i][j] << "\t";
		}
		cout << endl;
	}
	//*/
	

	populate_closest_nodes();

	///*
	for(int i = 0; i < closest_nodes.size(); i++) {
		cout << i << ": ";
		for(const auto j : closest_nodes[i]) {
			cout << j << " ";
		}
		cout << endl;
	}
	//*/
	

	

	cout << min_distance(unordered_set<string>(), 0, 0) << endl;

	return 0;
}
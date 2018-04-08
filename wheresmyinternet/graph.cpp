#include <iostream>
#include <vector>
#include <queue>
#include <unordered_set>

using namespace std;

int main() {
	int nodes, connections;

	cin >> nodes >> connections;

	vector<vector<int> > graph(nodes + 1, vector<int>());

	int temp1, temp2;

	for(int i = 0; i < connections; i++) {
		cin >> temp1 >> temp2;
		graph[temp1].push_back(temp2);
		graph[temp2].push_back(temp1);
	}

	queue<int> q;
	unordered_set<int> s;

	s.insert(1);
	// Start with 1
	for(auto i : graph[1]) {
		q.push(i);
	}
	// Clear 1
	graph[1].clear();

	while(!q.empty()) {
		temp1 = q.front();
		q.pop();
		s.insert(temp1);
		// Traverse graph
		for(auto i : graph[temp1]) {
			q.push(i);
		}
		// Clear x
		graph[temp1].clear();
	}

	if(s.size() == nodes) {
		cout << "Connected" << endl;
	} else {
		for(int i = 1; i <= nodes; i++) {
			if(s.find(i) == s.end()) {
				cout << i << endl;
			}
		}
	}

	return 0;
}
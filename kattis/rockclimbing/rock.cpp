#include <iostream>
#include <vector>
#include <queue>
#include <algorithm>

using namespace std;

struct Path {
	vector<int> current_point;
	int cost;
	int max_cost;

	Path() {
		current_point.push_back(0);
		current_point.push_back(0);
		cost = 0;
		max_cost = 0;
	}
};

struct myCompare {
	bool operator() (const Path & lhs, const Path & rhs) {
		return lhs.cost > rhs.cost;
	}
};

int main() {
	int rows;
	int columns;
	vector<vector<int> > grid;
	int temp;
	char junk;

	cin >> rows >> columns;

	// Skip "E"s
	for(int i = 0; i < columns; i++) {
		cin >> junk;
	}

	for(int i = 0; i < rows; i++) {
		grid.push_back(vector<int> (columns, 0));
		for(int j = 0; j < columns; j++) {
			cin >> temp;
			grid[i][j] = temp;
		}
	}

	// Skip "S"s
	for(int i = 0; i < columns; i++) {
		cin >> junk;
	}

	priority_queue<Path, vector<Path>, myCompare> pq;

	Path myPath;
	Path temp_path;
	vector<int> test_point(2, 0);
	int cost;

	// Add bottom row to prioirty queue
	for(int i = 0; i < columns; i++) {
		myPath.current_point[0] = i;
		myPath.current_point[1] = rows - 1;
		myPath.cost = grid[rows - 1][i];
		myPath.max_cost = myPath.cost;

		if(myPath.cost >= 0) {
			pq.push(myPath);
		}
	}

	while(!pq.empty()) {
		myPath = pq.top();
		pq.pop();

		cout << myPath.current_point[0] << " " << myPath.current_point[1] << " " << myPath.cost << endl;

		if(myPath.current_point[1] == 0) {
			break;
		}

		// Go left
		if(myPath.current_point[0] - 1 >= 0) {
			test_point[0] = myPath.current_point[0] - 1;
			test_point[1] = myPath.current_point[1];

			// Test cost
			cost = myPath.cost + grid[test_point[1]][test_point[0]];
			if(cost >= 0) {
				temp_path.current_point = test_point;
				temp_path.cost = cost;
				temp_path.max_cost = max(cost, myPath.max_cost);
				pq.push(temp_path);
			} else {
				break;
			}
		}

		// Go right
		if(myPath.current_point[0] + 1 < columns) {
			test_point[0] = myPath.current_point[0] + 1;
			test_point[1] = myPath.current_point[1];

			// Test cost
			cost = myPath.cost + grid[test_point[1]][test_point[0]];
			if(cost >= 0) {
				temp_path.current_point = test_point;
				temp_path.cost = cost;
				temp_path.max_cost = max(cost, myPath.max_cost);
				pq.push(temp_path);
			} else {
				break;
			}
		}

		// Go Down
		if(myPath.current_point[1] - 1 >= 0) {
			test_point[0] = myPath.current_point[0];
			test_point[1] = myPath.current_point[1] - 1;

			// Test cost
			cost = myPath.cost + grid[test_point[1]][test_point[0]];
			if(cost >= 0) {
				temp_path.current_point = test_point;
				temp_path.cost = cost;
				temp_path.max_cost = max(cost, myPath.max_cost);
				pq.push(temp_path);
			} else {
				break;
			}
		}

		// Go Up
		if(myPath.current_point[1] + 1 < rows) {
			test_point[0] = myPath.current_point[0];
			test_point[1] = myPath.current_point[1] + 1;

			// Test cost
			cost = myPath.cost + grid[test_point[1]][test_point[0]];
			if(cost >= 0) {
				temp_path.current_point = test_point;
				temp_path.cost = cost;
				temp_path.max_cost = max(cost, myPath.max_cost);
				pq.push(temp_path);
			} else {
				break;
			}
		}


	}

	cout << myPath.max_cost << endl;


	return 0;
}
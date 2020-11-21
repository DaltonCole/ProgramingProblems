#include <iostream>
#include <queue>
#include <vector>
#include <unordered_set>

using namespace std;

//unordered_set<vector<int> >::const_iterator got;

struct Path {
	int cost;
	unordered_set<vector<int> > past_points;

	/*
	bool been_there_done_that(int x, int y) {
		tuple<int, int> new_point (make_tuple(x, y));

	}
	*/
};

struct mycomparison
{
	bool operator() (const Path & lhs, const Path & rhs) {
		return lhs.cost > rhs.cost;
	}

};

int  main() {
	int rows, columns;
	int temp;
	vector<vector<int> > grid;
	char nothing;

	cin >> rows >> columns;

	for(int i = 0; i < columns; i++) {
		cin >> nothing;
	}

	// Read in cliff
	for(int i = 0; i < rows; i++) {
		grid.push_back(vector<int>());
		for(int j = 0; j < columns; j++) {
			cin >> temp;
			grid[i].push_back(temp);
		}
	}

	// Start
	for(int i = 0; i < columns; i++) {
		cin >> nothing;
	}

	priority_queue<Path, vector<Path>, mycomparison> pq;

	// Initalize priority queue with last row
	Path path;
	Path temp_path;
	int weight;

	/*
	for(int i = 0; i < columns; i++) {
		weight = grid[rows-1][i];
		dot.x = rows-1;
		dot.y = i;
		path.cost = weight;
		path.point = dot;
		pq.push(path);
	}

	while(pq.empty() == false) {
		path = pq.top();
		pq.pop();

		cout << path.point.x << " " << path.point.y << " " << path.cost << endl;


		// See if going up (reducing x by 1) results in top of cliff
		if(path.point.x == 0) {
			break;
		}

		// Go left
		if(path.point.x - 1 >= 0) {
			weight = grid[path.point.x - 1][path.point.y];
			if(path.cost + weight >= 0) {
				temp_path.cost = path.cost + weight;
				temp_path.point.x = path.point.x - 1;
				temp_path.point.y = path.point.y;
				pq.push(temp_path);
			}
		}

		// Go right
		if(path.point.x + 1 < rows) {
			weight = grid[path.point.x + 1][path.point.y];
			if(path.cost + weight >= 0) {
				temp_path.cost = path.cost + weight;
				temp_path.point.x = path.point.x + 1;
				temp_path.point.y = path.point.y;
				pq.push(temp_path);
			}
		}

		// Go Up
		if(path.point.y - 1 >= 0) {
			weight = grid[path.point.x][path.point.y - 1];
			if(path.cost + weight >= 0) {
				temp_path.cost = path.cost + weight;
				temp_path.point.x = path.point.x;
				temp_path.point.y = path.point.y - 1;
				pq.push(temp_path);
			}
		}

		// Go Down
		if(path.point.y + 1 < columns) {
			weight = grid[path.point.x][path.point.y + 1];
			if(path.cost + weight >= 0) {
				temp_path.cost = path.cost + weight;
				temp_path.point.x = path.point.x;
				temp_path.point.y = path.point.y + 1;
				pq.push(temp_path);
			}
		}

	}
	*/

	//cout << path.cost << endl;

	/*
	for(auto i : grid) {
		for(auto j: i) {
			cout << j << " ";
		}
		cout << endl;
	}
	*/

	return 0;
}
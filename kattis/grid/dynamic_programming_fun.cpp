#include <iostream>
#include <vector>
#include <queue>
#include <utility>
#include <bits/stdc++.h>

using namespace std;

int main() {
	int rows;
	int columns;
	string line;

	cin >> rows >> columns;

	vector<vector<int> > grid(rows, vector<int>(columns));
	vector<vector<int> > moves(rows, vector<int>(columns, INT_MAX));

	
	for(int i = 0; i < rows; i++) {
		cin >> line;
		for(int j = 0; j < columns; j++) {
			grid[i][j] = line[j] - 48;
		}
	}

	// Queue of x,y coordinates to look at
	queue<pair<int, int> > coor_q;
	pair<int, int> space;
	int x, y;
	int jump_distance;

	coor_q.push(make_pair(0, 0));
	moves[0][0] = 0;

	while(coor_q.empty() == false) {
		space = coor_q.front();
		coor_q.pop();

		x = space.first;
		y = space.second;

		jump_distance = grid[x][y];


		// Go up
		if(x - jump_distance >= 0) {
			if(moves[x - jump_distance][y] > moves[x][y] + 1) {
				moves[x - jump_distance][y] = moves[x][y] + 1;
				coor_q.push(make_pair(x - jump_distance, y));
			}
		}
		// Go down
		if(x + jump_distance < rows) {
			if(moves[x + jump_distance][y] > moves[x][y] + 1) {
				moves[x + jump_distance][y] = moves[x][y] + 1;
				coor_q.push(make_pair(x + jump_distance, y));
			}
		}
		// Go left
		if(y - jump_distance >= 0) {
			if(moves[x][y - jump_distance] > moves[x][y] + 1) {
				moves[x][y - jump_distance] = moves[x][y] + 1;
				coor_q.push(make_pair(x, y - jump_distance));
			}
		}
		// Go right
		if(y + jump_distance < columns) {
			if(moves[x][y + jump_distance] > moves[x][y] + 1) {
				moves[x][y + jump_distance] = moves[x][y] + 1;
				coor_q.push(make_pair(x, y + jump_distance));
			}
		}
	}

	if(moves[rows-1][columns-1] == INT_MAX) {
		cout << "-1" << endl;
	} else {
		cout << moves[rows-1][columns-1] << endl;
	}


	return 0;
}
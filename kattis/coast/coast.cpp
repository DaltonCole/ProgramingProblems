#include <iostream>
#include <vector>

using namespace std;

// Convert all oceans to 2 by looking at the edges of the map and propogating
void on_grid(vector<vector<int> >& grid, const int x, const int y) {
	if(grid[x][y] == 0) {
		grid[x][y] = 2;
	} else {
		return;
	}


	// Go left
	if(x > 0) {
		on_grid(grid, x - 1, y);
	}
	// Go right
	if(x < grid.size() - 1) {
		on_grid(grid, x + 1, y);
	}
	// Go up
	if(y > 0) {
		on_grid(grid, x, y - 1);
	}
	// Go down
	if(y < grid[0].size() - 1) {
		on_grid(grid, x, y + 1);
	}

	return;
}

// For each 1, look around it and add coast if on edge or near a 2
int make_coast(vector<vector<int> >& grid) {
	int coast = 0;
	for(int i = 0; i < grid.size(); i++) {
		for(int j = 0; j < grid[0].size(); j++) {
			// If land
			if(grid[i][j] == 1) {
				// Check up
				if(j > 0) {
					if(grid[i][j - 1] == 2) {
						coast++;
						//grid[i][j - 1] = 3;
					}
				} else {
					coast++;
				}
				// Check down
				if(j < grid[0].size() - 1) {
					if(grid[i][j + 1] == 2) {
						coast++;
						//grid[i][j + 1] = 3;
					}
				} else {
					coast++;
				}
				// Check left
				if(i > 0) {
					if(grid[i - 1][j] == 2) {
						coast++;
						//grid[i - 1][j] = 3;
					}
				} else {
					coast++;
				}
				// Check right
				if(i < grid.size() - 1) {
					if(grid[i + 1][j] == 2) {
						coast++;
						//grid[i + 1][j] = 3;
					}
				} else {
					coast++;
				}
			}
		}
	}

	return coast;
}

int main() {
	int rows, columns;

	cin >> rows >> columns;

	vector<vector<int> > grid(rows, vector<int>(columns, 0));

	string s;

	for(auto& r : grid) {
		cin >> s;
		for(int j = 0; j < columns; j++) {
			r[j] = s[j] - 48;
		}
	}

	// --- Go around the perimeter and mark ocean as 2 --- //
	// Left side
	for(int i = 0; i < rows; i++) {
		on_grid(grid, i, 0);
	}
	// right side
	for(int i = 0; i < rows; i++) {
		on_grid(grid, i, columns - 1);
	}
	// Top
	for(int i = 0; i < columns; i++) {
		on_grid(grid, 0, i);
	}
	// Bottom
	for(int i = 0; i < columns; i++) {
		on_grid(grid, rows - 1, i);
	}

	/*
	for(auto& r : grid) {
		for(auto& c : r) {
			cout << c;
		}
		cout << endl;
	}
	*/

	// Make coast
	cout << make_coast(grid) << endl;

	
}
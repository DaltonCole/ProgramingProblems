#include <iostream>
#include <vector>

using namespace std;

void change_grid(vector<vector<char> >& grid, int x, int y, bool change) {
	// Update counted stars to be '*'
	if(grid[x][y] == '-') {
		// Change if change is set
		if(change == true) {
			grid[x][y] = '*';
		} else {
			grid[x][y] = '+';
		}

		// Go left
		if(x > 0) {
			change_grid(grid, x - 1, y, true);
		}
		// Go right
		if(x < grid.size() - 1) {
			change_grid(grid, x + 1, y, true);
		}
		// Go up
		if(y > 0) {
			change_grid(grid, x, y - 1, true);
		}
		// Go down
		if(y < grid[0].size() - 1) {
			change_grid(grid, x, y + 1, true);
		}
	}

	if(change == false) {
		// Go to next space
		if(y == grid[0].size() - 1) {
			// Wrap around
			if(x != grid.size() - 1) {
				change_grid(grid, x + 1, 0, false);
			}
		} else {
			change_grid(grid, x, y + 1, false);
		}
	}
	

	return;
}

int star_count(const vector<vector<char> >& grid) {
	int count = 0;
	for(auto& r : grid) {
		for(auto& c : r) {
			if(c == '+') {
				count++;
			}
		}
	}
	return count;
}

int main() {
	int rows, columns;
	string s;
	int case_num = 0;

	while(cin >> rows >> columns) {
		case_num++;
		vector<vector<char> > grid(rows, vector<char>(columns, 'a'));

		for(auto& r : grid) {
			cin >> s;
			for(int i = 0; i < columns; i++) {
				r[i] = s[i];
			}
		}

		change_grid(grid, 0, 0, false);

		cout << "Case " << case_num << ": " << star_count(grid) << endl;
	}

	return 0;
}
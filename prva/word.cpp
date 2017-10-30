#include <iostream>
#include <vector>

using namespace std;

int main() {
	int row;
	int col;
	string line;
	char c;
	string best_word;

	vector<vector<char> > grid;

	cin >> row >> col;

	for(int i = 0; i < row; i++) {
		grid.push_back(vector<char> (col, '#'));
		cin >> line;
		for(int j = 0; j < col; j++) {
			grid[i][j] = line[j];
		}
	}

	
	/*
	for(auto i : grid) {
		for(auto j : i) {
			cout << j;
		}
		cout << endl;
	}
	*/

	best_word = "";
	for(int i = 0; i < row; i++) {
		for(int j = 0; j < col; j++) {
			best_word += 'z';
		}
	}
	

	// Check from left to right
	for(int i = 0; i < row; i++) {
		line = "";
		for(int j = 0; j < col; j++) {
			if(grid[i][j] == '#') {
				if(line.length() >= 2) {
					if(line < best_word) {
						best_word = line;
					}
				}
				line = "";
			} else if(j == col - 1) {
				line += grid[i][j];
				if(line.length() >= 2) {
					if(line < best_word) {
						best_word = line;
					}
				}
				line = "";
			} else {
				line += grid[i][j];
			}
		}
	}

	// Check from top to bottom
	for(int j = 0; j < col; j++) {
		line = "";
		for(int i = 0; i < row; i++) {
			if(grid[i][j] == '#') {
				if(line.length() >= 2) {
					if(line < best_word) {
						best_word = line;
					}
				}
				line = "";
			} else if(i == row - 1) {
				line += grid[i][j];
				if(line.length() >= 2) {
					if(line < best_word) {
						best_word = line;
					}
				}
				line = "";
			} else {
				line += grid[i][j];
			}
		}
	}

	cout << best_word << endl;

	return 0;
}
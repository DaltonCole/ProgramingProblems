#include <iostream>

using namespace std;

int grid[1001][1001];
int color[1001][1001];
int row;
int col;

void label(int x, int y, int lab, int orig) {
	// If out of bounds, return
	if(x < 0 || y < 0 || x >= row || y >= col) {
		return;
	}
	// If not same color, return
	if(orig != grid[x][y]) {
		return;
	}
	// If we have already been here, return
	if(color[x][y] != 0) {
		return;
	}

	color[x][y] = lab;

	// Go left
	label(x-1, y, lab, orig);
	// Go right
	label(x+1, y, lab, orig);
	// Go down
	label(x, y-1, lab, orig);
	// Go up
	label(x, y+1, lab, orig);

	return;
}

int main() {
	int temp;
	string line;

	cin >> row >> col;

	for(int i = 0; i < row; i++) {
		cin >> line;
		for(int j = 0; j < col; j++) {
			grid[i][j] = line[j] - '0';
			color[i][j] = 0;
		}
	}

	// Color graph
	int lab = 1;
	for(int i = 0; i < row; i++) {
		for(int j = 0; j < col; j++) {
			if(color[i][j] == 0) {
				lab++;
				label(i, j, lab, grid[i][j]);
			}
		}
	}

	int cases;
	cin >> cases;
	int x1, x2, y1, y2;
	bool dec;
	bool bin;

	for(int i = 0; i < cases; i++) {
		cin >> x1 >> y1 >> x2 >> y2;
		x1--;
		x2--;
		y1--;
		y2--;

		dec = false;
		bin = false;

		if(grid[x1][y1] == 0 && grid[x2][y2] == 0) {
			if(color[x1][y1] == color[x2][y2]) {
				bin = true;
			}
		}

		if(grid[x1][y1] == 1 && grid[x2][y2] == 1) {
			if(color[x1][y1] == color[x2][y2]) {
				dec = true;
			}
		}

		if(bin == false && dec == false) {
			cout << "neither" << endl;
		} else if (bin == true) {
			cout << "binary" << endl;
		} else {
			cout << "decimal" << endl;
		}
	}

	return 0;
}
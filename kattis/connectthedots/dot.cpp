#include <iostream>
#include <vector>
#include <map>
#include <algorithm>

using namespace std;

struct Point {
	int x;
	int y;
};

int main() {
	vector<string> grid;
	map<int, Point> mapping;
	Point coor;

	string line;
	int index;

	bool next = false;

	while(getline(cin, line)) {
		if(line == "") {
			// Do stuff
			for(auto p : mapping) {
				next = false;
				for(auto s : mapping) {
					if(s.first <= p.first) {
						continue;
					} else if(next == true) {
						break;
					}
					next = true;
					// Go horizontally
					if(p.second.y == s.second.y) {
						for(int h = min(p.second.x, s.second.x); h < max(p.second.x, s.second.x); h++) {
							if(grid[p.second.y][h] == '.') {
								grid[p.second.y][h] = '-';
							} else if (grid[p.second.y][h] == '|') {
								grid[p.second.y][h] = '+';
							}
						}
					} else if(p.second.x == s.second.x) { // Go vertically
						for(int h = min(p.second.y, s.second.y); h < max(p.second.y, s.second.y); h++) {
							if(grid[h][p.second.x] == '.') {
								grid[h][p.second.x] = '|';
							} else if (grid[h][p.second.x] == '-') {
								grid[h][p.second.x] = '+';
							}
						}
					}
				}
			}
			for(auto r : grid) {
				cout << r << endl;
			}
			cout << endl;

			// Clear
			grid.clear();
			mapping.clear();
		} else {
			grid.push_back(line);
			for(int i = 0; i < line.length(); i++) {
				if(line[i] != '.') {
					coor.y = grid.size() - 1;
					coor.x = i;
					index = line[i];
					if(line[i] > '9' && line[i] < 'a') {
						index += 100;
					}
					mapping[index] = coor;
				}
			}
		}
	}

	// Do stuff
			for(auto p : mapping) {
				next = false;
				for(auto s : mapping) {
					if(s.first <= p.first) {
						continue;
					} else if(next == true) {
						break;
					}
					next = true;
					// Go horizontally
					if(p.second.y == s.second.y) {
						for(int h = min(p.second.x, s.second.x); h < max(p.second.x, s.second.x); h++) {
							if(grid[p.second.y][h] == '.') {
								grid[p.second.y][h] = '-';
							} else if (grid[p.second.y][h] == '|') {
								grid[p.second.y][h] = '+';
							}
						}
					} else if(p.second.x == s.second.x) { // Go vertically
						for(int h = min(p.second.y, s.second.y); h < max(p.second.y, s.second.y); h++) {
							if(grid[h][p.second.x] == '.') {
								grid[h][p.second.x] = '|';
							} else if (grid[h][p.second.x] == '-') {
								grid[h][p.second.x] = '+';
							}
						}
					}
				}
			}
			for(auto r : grid) {
				cout << r << endl;
			}
			cout << endl;

	return 0;
}
#include <iostream>
#include <unordered_set>
#include <vector>
#include <string> 
#include <unordered_map>

using namespace std;

int main() {
	int rows, columns;
	string line;
	string sub;
	bool in_a_set;
	int set_number;

	vector<unordered_set<string> > zeros;
	vector<unordered_set<string> > ones;
	vector<string> grid;

	cin >> rows >> columns;

	for(int i = 0; i < rows; i++) {
		cin >> line;
		grid.push_back(line);
		for(int j = 0; j < columns; j++) {
			in_a_set = false;
			set_number = 0;
			// Check up
			if(i != 0) {
				// Zero
				if(grid[i][j] == '0' && grid[i-1][j] == '0') {
					for(int k = 0; k < zeros.size(); k++) {
						sub = to_string(i-1) + " " + to_string(j);
						auto got = zeros[k].find(sub);
						if(got != zeros[k].end()) {
							in_a_set = true;
							set_number = k;
							zeros[k].insert(to_string(i) + " " + to_string(j));
							break;
						}
					}
				}
				// One
				if(grid[i][j] == '1' && grid[i-1][j] == '1') {
					for(int k = 0; k < ones.size(); k++) {
						sub = to_string(i-1) + " " + to_string(j);
						auto got = ones[k].find(sub);
						if(got != ones[k].end()) {
							in_a_set = true;
							set_number = k;
							ones[k].insert(to_string(i) + " " + to_string(j));
							break;
						}
					}
				}
			}
			// Check left
			if(j != 0) {
				// Zero
				if(grid[i][j] == '0' && grid[i][j-1] == '0') {
					for(int k = 0; k < zeros.size(); k++) {
						sub = to_string(i) + " " + to_string(j-1);
						auto got = zeros[k].find(sub);
						//for(auto m : zeros[k]) {
						//	cout << m << " . ";
						//}
						//cout << endl;
						if(got != zeros[k].end()) {
							//cout << in_a_set << endl;
							if(set_number != k && in_a_set) {
								//cout << "test" << endl;
								zeros[k].insert(zeros[set_number].begin(), zeros[set_number].end());
								zeros.erase(zeros.begin() + set_number);
							} else {
								in_a_set = true;
								zeros[k].insert(to_string(i) + " " + to_string(j));
							}
							break;
						}
					}
				}
				// One
				if(grid[i][j] == '1' && grid[i][j-1] == '1') {
					//cout << "a" << endl;
					//cout << ones.size() << endl;
					for(int k = 0; k < ones.size(); k++) {
						sub = to_string(i) + " " + to_string(j-1);
						//cout << sub << endl;
						auto got = ones[k].find(sub);
						if(got != ones[k].end()) {
							if(set_number != k && in_a_set) {
								ones[k].insert(ones[set_number].begin(), ones[set_number].end());
								ones.erase(ones.begin() + set_number);
							} else {
								in_a_set = true;
								ones[k].insert(to_string(i) + " " + to_string(j));
							}
							break;
						}
					}
				}
			}
			if(in_a_set == false) {
				if(grid[i][j] == '0') {
					zeros.push_back(unordered_set<string>());
					zeros[zeros.size() - 1].insert(to_string(i) + " " + to_string(j));
				} else {
					ones.push_back(unordered_set<string>());
					ones[ones.size() - 1].insert(to_string(i) + " " + to_string(j));
				}
			}
		}
	}

	/*
	for(auto i : zeros) {
		for(auto j : i) {
			cout << j << " | ";
		}
		cout << endl;
	}
	cout << zeros.size() << endl;

	for(auto i : ones) {
		for(auto j : i) {
			cout << j << " | ";
		}
		cout << endl;
	}
	cout << ones.size() << endl;
	*/

	unordered_map<string, int> zero_map;
	unordered_map<string, int> one_map;

	for(int i = 0; i < zeros.size(); i++) {
		for(auto s : zeros[i]) {
			zero_map[s] = i + 1;
		}
	}

	for(int i = 0; i < ones.size(); i++) {
		for(auto s : ones[i]) {
			one_map[s] = i + 1;
		}
	}


	int cases;
	cin >> cases;
	int x1, x2, y1, y2;
	bool b, d;
	string first;
	string second;

	for(int i = 0; i < cases; i++) {
		cin >> x1 >> y1 >> x2 >> y2;
		b = false;
		d = false;
		x1--;
		x2--;
		y1--;
		y2--;

		first =  to_string(x1) + " " + to_string(y1);
		second =  to_string(x2) + " " + to_string(y2);

		if(zero_map[first] == zero_map[second] && zero_map[first] != 0) {
			b = true;
		}
		if(one_map[first] == one_map[second] && one_map[first] != 0) {
			d = true;
		}

		if(b == false && d == false) {
			cout << "neither" << endl;
		} else if (b == true) {
			cout << "binary" << endl;
		} else if(d == true) {
			cout << "decimal" << endl;
		}
	}

	return 0;
}
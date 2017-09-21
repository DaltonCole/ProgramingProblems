#include <iostream>
#include <cmath>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
	int test_cases;

	char start_col;
	char end_col;
	int start_row;
	int end_row;

	vector<vector<int> > diagnal;
	vector<vector<int> > other_diagnal;
	vector<vector<int> > v(10);

	vector<int> place = {0, 0};
	bool go_to_next;


	cin >> test_cases;

	for(int i = 0; i < test_cases; i++) {
		cin >> start_col >> start_row >> end_col >> end_row;
		/*
		cout << start_col << endl;
		cout << start_row << endl;
		cout << end_col << endl;
		cout << end_row << endl;
		cout << endl;
		*/

		start_col -= 65;
		end_col -= 65;
		start_row -= 1;
		end_row -= 1;

		/*
		cout << start_col << endl;
		cout << start_row << endl;
		cout << end_col << endl;
		cout << end_row << endl;
		cout << endl << endl;
		*/

		if(int(abs(start_col - end_col) + abs(start_row - end_row)) % 2 == 1){
			cout << "Impossible" << endl;
			continue;
		}

		// Distance of 0
		if(start_col == end_col and start_row == end_row) {
			cout << 0 << " " << char(start_col + 65) << " " << start_row + 1 << endl;
			continue;
		}

		// Distance of 1
		if(abs(start_col - end_col) == abs(start_row - end_row)) {
			cout << 1 << " " << char(start_col + 65) << " " << start_row + 1;
			cout << " " << char(end_col + 65) << " " << end_row + 1 << endl;
			continue;
		}

		// Distance of 2
		for(int l = 1; l < 8; l++) {
			if(start_col + l < 8 and start_row + l < 8) {
				place[0] = start_col + l;
				place[1] = start_row + l;
				diagnal.push_back(place);
			}
			if(start_col + l < 8 and start_row - l >= 0) {
				place[0] = start_col + l;
				place[1] = start_row - l;
				diagnal.push_back(place);
			}
			if(start_col - l >= 0 and start_row + l < 8) {
				place[0] = start_col - l;
				place[1] = start_row + l;
				diagnal.push_back(place);
			}
			if(start_col - l >= 0 and start_row - l >= 0) {
				place[0] = start_col - l;
				place[1] = start_row - l;
				diagnal.push_back(place);
			}

			if(end_col + l < 8 and end_row + l < 8) {
				place[0] = end_col + l;
				place[1] = end_row + l;
				other_diagnal.push_back(place);
			}
			if(end_col + l < 8 and end_row - l >= 0) {
				place[0] = end_col + l;
				place[1] = end_row - l;
				other_diagnal.push_back(place);
			}
			if(end_col - l >= 0 and end_row + l < 8) {
				place[0] = end_col - l;
				place[1] = end_row + l;
				other_diagnal.push_back(place);
			}
			if(end_col - l >= 0 and end_row - l >= 0) {
				place[0] = end_col - l;
				place[1] = end_row - l;
				other_diagnal.push_back(place);
			}
		}

		/*
		for(auto a : diagnal) {
			cout << a[0] << " " << a[1] << endl;
		}
		cout << endl;
		for(auto a : other_diagnal) {
			cout << a[0] << " " << a[1] << endl;
		}
		cout << endl;
		*/

		go_to_next = false;
		for(auto spot1 : diagnal) {
			if(go_to_next == true) {
				break;
			}
			for(auto spot2 : other_diagnal) {
				if(spot1[0] == spot2[0] and spot1[1] == spot2[1]) {
					cout << 2 << " " << char(start_col + 65) << " " << start_row + 1;
					cout << " " << char(spot1[0] + 65) << " " << spot1[1] + 1;
					cout << " " << char(end_col + 65) << " " << end_row + 1 << endl; 
					go_to_next = true;
					break;
				}
			}
		}

		diagnal.clear();
		other_diagnal.clear();
	}
	return 0;
}
#include <iostream>
#include <vector>
using namespace std;

int main() {
	int total = 0;
	int y_axis = 0;
	string input;

	vector<char> temp(7, ' ');
	vector<vector<char> > board(7, temp);

	while(cin >> input) {
		if(input.size() == 3) {
			for(int i = 0; i < input.size(); i++) {
				board[i+2][y_axis] = input[i];
			}
		} else {
			for(int i = 0; i < input.size(); i++) {
				board[i][y_axis] = input[i];
			}
		}
		y_axis++;
	}
	/*
	for(int i = 0; i < 7; i++){
		for(int j = 0; j < 7; j++) {
			cout << board[i][j] << " ";
		}
		cout << endl;
	}
	*/

	for(int i = 0; i < 7; i++) {
		for(int j = 0; j < 7; j++) {
			if(board[i][j] == 'o') {
				// Up
				if(i > 1) {
					if(board[i-2][j] == '.' && board[i-1][j] == 'o'){
						total++;
					}
				}
				// Down
				if(i < 5) {
					if(board[i+2][j] == '.' && board[i+1][j] == 'o'){
						total++;
					}
				}
				// Left
				if(j > 1) {
					if(board[i][j-2] == '.' && board[i][j-1] =='o'){
						total++;
					}
				}
				// Right
				if(j < 5) {
					if(board[i][j+2] == '.' && board[i][j+1] == 'o'){
						total++;
					}
				}
			}
		}
	}

	cout << total << endl;

	return 0;
}
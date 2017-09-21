#include <iostream>
#include <queue>
#include <vector>
#include <algorithm>

using namespace std;

vector< vector<char> > ideal_board = 
{
	{'1', '1', '1', '1', '1'},
	{'0', '1', '1', '1', '1'},
	{'0', '0', ' ', '1', '1'},
	{'0', '0', '0', '0', '1'},
	{'0', '0', '0', '0', '0'},
	{'2', '2', '-', '-', '-'}
};

void find_space(vector< vector<char> > & board) {
	for(int i = 0; i < 5; i++) {
		for(int j = 0; j < 5; j++) {
			if(board[i][j] == ' ') {
				board[5][0] = i + 48;
				board[5][1] = j + 48;
				return;
			}
		}
	}
}

int bfs(vector< vector<char> > & board) {
	if(board == ideal_board) {
		return 0;
	}

	queue<vector<vector<char>>> q1;
	vector< vector<char> > temp;
	int depth = 0;
	int x;
	int y;
	int last_size;

	q1.push(board);

	for(int i = 0; i < 10; i++) {
		depth++;
		//cout << depth << endl;

		last_size = q1.size();
		for(int j = 0; j < last_size; j++) {
			board = q1.front();
			q1.pop();
			x = board[5][0] - 48;
			y = board[5][1] - 48;
			// left left up
			if(x - 2 >= 0 and y + 1 < 5) {
				temp = board;
				swap(temp[x][y], temp[x-2][y+1]);
				temp[5][0] -= 2;
				temp[5][1] += 1;
				if(temp == ideal_board) {
					return depth;
				}
				q1.push(temp);
			}
			// left left down
			if(x - 2 >= 0 and y - 1 >= 0) {
				temp = board;
				swap(temp[x][y], temp[x-2][y-1]);
				temp[5][0] -= 2;
				temp[5][1] -= 1;
				if(temp == ideal_board) {
					return depth;
				}
				q1.push(temp);
			}
			// left up up
			if(x - 1 >= 0 and y + 2 < 5) {
				temp = board;
				swap(temp[x][y], temp[x-1][y+2]);
				temp[5][0] -= 1;
				temp[5][1] += 2;
				if(temp == ideal_board) {
					return depth;
				}
				q1.push(temp);
			}
			// left down down
			if(x - 1 >= 0 and y - 2 >= 0) {
				temp = board;
				swap(temp[x][y], temp[x-1][y-2]);
				temp[5][0] -= 1;
				temp[5][1] -= 2;
				if(temp == ideal_board) {
					return depth;
				}
				q1.push(temp);
			}
			// right right up
			if(x + 2 < 5 and y + 1 < 5) {
				temp = board;
				swap(temp[x][y], temp[x+2][y+1]);
				temp[5][0] += 2;
				temp[5][1] += 1;
				if(temp == ideal_board) {
					return depth;
				}
				q1.push(temp);
			}
			// right right down
			if(x + 2 < 5 and y - 1 >= 0) {
				temp = board;
				swap(temp[x][y], temp[x+2][y-1]);
				temp[5][0] += 2;
				temp[5][1] -= 1;
				if(temp == ideal_board) {
					return depth;
				}
				q1.push(temp);
			}
			// right up up
			if(x + 1 < 5 and y + 2 < 5) {
				temp = board;
				swap(temp[x][y], temp[x+1][y+2]);
				temp[5][0] += 1;
				temp[5][1] += 2;
				if(temp == ideal_board) {
					return depth;
				}
				q1.push(temp);
			}
			// right down down
			if(x + 1 < 5 and y - 2 >= 0) {
				temp = board;
				swap(temp[x][y], temp[x+1][y-2]);
				temp[5][0] += 1;
				temp[5][1] -= 2;
				if(temp == ideal_board) {
					return depth;
				}
				q1.push(temp);
			}
		}
	}

	return -1;
}

int main() {
	int test_cases;
	vector< vector<char> > board;
	string input;
	int depth;

	for(int i = 0; i < 5; i++) {
		board.push_back(vector<char>(5, 'a'));
	}
	board.push_back(vector<char>(5, '-'));

	cin >> test_cases;
	getline(cin, input);

	for(int i = 0; i < test_cases; i++) {
		for(int lines = 0; lines < 5; lines++) {
			getline(cin, input);
			for(int character = 0; character < 5; character++) {
				board[lines][character] = input[character];
			}
		}
		find_space(board);

		depth = bfs(board);

		if(depth == -1) {
			cout << "Unsolvable in less than 11 move(s)." << endl;
		} else {
			cout << "Solvable in " << depth << " move(s)." << endl;
		}
	}

	/*
	for(int t = 0; t < 5; t++) {
		for(int j = 0; j < 5; j++) {
			cout << ideal_board[t][j];
		}
		cout << endl;
	}
	*/

	return 0;
}
#include <iostream>
using namespace std;

int main() {
	int board[4][4];
	int input;

	for(int i = 0; i < 4; i++) { // row
		for(int j = 0; j < 4; j++) { // column
			cin >> input;
			board[i][j] = input;
		}
	}

	cin >> input;

	if(input == 0) { // left
		for(int k = 0; k < 4; k++) {
			for(int i = 0; i < 4; i++) {
				for(int j = 1; j < 4; j++) {
					if(board[i][j] == 0) {
						continue;
					} else if(board[i][j-1] == 0) {
						board[i][j-1] = board[i][j];
						board[i][j] = 0;
					}
				}
			}
		}

			for(int i = 0; i < 4; i++) {
				for(int j = 1; j < 4; j++) {
					if(board[i][j-1] == board[i][j]) {
						board[i][j-1] *= 2;
						board[i][j] = 0;
					}
				}
		}

		for(int k = 0; k < 4; k++) {
			for(int i = 0; i < 4; i++) {
				for(int j = 1; j < 4; j++) {
					if(board[i][j] == 0) {
						continue;
					} else if(board[i][j-1] == 0) {
						board[i][j-1] = board[i][j];
						board[i][j] = 0;
					}
				}
			}
		}

	} else if(input == 1) { // up
		for(int k = 0; k < 4; k++) {
			for(int i = 1; i < 4; i++) {
				for(int j = 0; j < 4; j++) {
					if(board[i][j] == 0) {
						continue;
					} else if(board[i-1][j] == 0) {
						board[i-1][j] = board[i][j];
						board[i][j] = 0;
					}
				}
			}
		}
			for(int i = 1; i < 4; i++) {
				for(int j = 0; j < 4; j++) {
					if(board[i-1][j] == board[i][j]) {
						board[i-1][j] *= 2;
						board[i][j] = 0;
					}
				}
			}
		for(int k = 0; k < 4; k++) {
			for(int i = 1; i < 4; i++) {
				for(int j = 0; j < 4; j++) {
					if(board[i][j] == 0) {
						continue;
					} else if(board[i-1][j] == 0) {
						board[i-1][j] = board[i][j];
						board[i][j] = 0;
					}
				}
			}
		}

	} else if (input == 2) { // right
		for(int k = 0; k < 4; k++) {
			for(int i = 0; i < 4; i++) {
				for(int j = 0; j < 3; j++) {
					if(board[i][j] == 0) {
						continue;
					} else if(board[i][j+1] == 0) {
						board[i][j+1] = board[i][j];
						board[i][j] = 0;
					}
				}
			}
		}
			for(int i = 0; i < 4; i++) {
				for(int j = 2; j >= 0; j--) {
					if(board[i][j+1] == board[i][j]) {
						board[i][j+1] *= 2;
						board[i][j] = 0;
					}
				}
			}
		for(int k = 0; k < 4; k++) {
			for(int i = 0; i < 4; i++) {
				for(int j = 0; j < 3; j++) {
					if(board[i][j] == 0) {
						continue;
					} else if(board[i][j+1] == 0) {
						board[i][j+1] = board[i][j];
						board[i][j] = 0;
					}
				}
			}
		}

	} else if (input == 3) { // down
		for(int k = 0; k < 4; k++) {
			for(int i = 0; i < 3; i++) {
				for(int j = 0; j < 4; j++) {
					if(board[i][j] == 0) {
						continue;
					} else if(board[i+1][j] == 0) {
						board[i+1][j] = board[i][j];
						board[i][j] = 0;
					}
				}
			}
		}
			for(int i = 2; i >= 0; i--) {
				for(int j = 0; j < 4; j++) {
					if(board[i+1][j] == board[i][j]) {
						board[i+1][j] *= 2;
						board[i][j] = 0;
					}
				}
			}
		for(int k = 0; k < 4; k++) {
			for(int i = 0; i < 3; i++) {
				for(int j = 0; j < 4; j++) {
					if(board[i][j] == 0) {
						continue;
					} else if(board[i+1][j] == 0) {
						board[i+1][j] = board[i][j];
						board[i][j] = 0;
					}
				}
			}
		}
	}



	for(int i = 0; i < 4; i++) {
		for(int j = 0; j < 4; j++) {
			cout << board[i][j] << " ";
		}
		cout << endl;
	}

	return 0;
}
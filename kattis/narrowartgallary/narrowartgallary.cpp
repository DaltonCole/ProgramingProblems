#include <iostream>
#include <<climits>

using namespace std;

typedef long long ll

// Remember, might be off by 1
const int MAX_ROWS = 200;
const int MIN_ROWS = 3;

const ll INF = LLONG_MAX;
const ll NEG_INF = LLONG_MIN;

// (Row, Column)
int hallway[MAX_ROWS][2];

ll top_down(const int row, const int rooms_to_close) {
	// If out no more rows to look at and still have rooms to close
	if(row == -1 && rooms_to_close != 0) {
		// Return
		return INT_MIN;
	}
}

ll narrow_art_gallary(const int rows, const int closed_rooms) {
	int temp;
	for(int i = 0; i < rows; i++) {
		cin >> temp;
		hallway[i][0] = temp;
		cin >> temp;
		hallway[i][1] = temp;
	}
}


int main() {
	int rows, closed_rooms;

	while(true) {
		cin >> rows >> closed_rooms;

		if(rows == 0) {
			break;
		}

		cout << narrow_art_gallary(rows, closed_rooms) << endl;
	}

	return 0;
}
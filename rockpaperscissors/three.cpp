#include <iostream>
#include <unordered_map>

using namespace std;

int winner(char & a, char & b);

struct Score {
	float wins;
	float loses;
};

int main() {
	int players;
	int games;

	unordered_map<int, Score> tracker;
	Score now;

	int player_a;
	int player_b;
	string play_a;
	string play_b;

	int victor;
	float percent;

	cout.setf(ios::fixed,ios::floatfield);
    cout.precision(3);

	while(true) {
		cin >> players;
		if(players == 0) {
			break;
		} else {
			tracker.clear();
		}
		cin >> games;

		for(int i = 0; i < (games * players * (players-1) >> 1); i++) {
			cin >> player_a >> play_a >> player_b >> play_b;
			victor = winner(play_a[0], play_b[0]);

			if(victor == 1) {
				tracker[player_a].wins++;
				tracker[player_b].loses++;
			} else if(victor == -1) {
				tracker[player_b].wins++;
				tracker[player_a].loses++;
			}
		}

		for(int i = 1; i <= players; i++) {
			if(tracker[i].wins == 0 && tracker[i].loses == 0) {
				cout << "-" << endl;
			} else {
				percent = tracker[i].wins / (tracker[i].loses + tracker[i].wins);
				cout << percent << endl;
			}
		}
		cout << endl;
	}
}


int winner(char & a, char & b) {
	if(a == b) {
		return 0;
	}
	// Rock vs paper
	if(a == 'r' && b == 'p') {
		return -1;
	}
	if(b == 'r' && a == 'p') {
		return 1;
	}

	// Paper vs scissors 
	if(a == 's' && b == 'p') {
		return 1;
	}
	if(b == 's' && a == 'p') {
		return -1;
	}

	// Scissors vs rock
	if(a == 's' && b == 'r') {
		return -1;
	}
	if(b == 's' && a == 'r') {
		return 1;
	}

	return 0;
}
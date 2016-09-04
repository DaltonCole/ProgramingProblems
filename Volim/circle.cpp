#include <iostream>
using namespace std;

int main() {

	int current_player = 0;
	int current_time = 0;
	int number_of_questions = 0;
	int player_time = 0;
	char answer;
	int loser = 0;

	cin >> current_player;

	cin >> number_of_questions;

	for(int i = 0; i < number_of_questions; i++) {
		cin >> player_time >> answer;

		current_time += player_time;

		if(current_time >= 210 && loser == 0) {
			loser = (current_player - 1 % 8) + 1;
		}

		if(answer == 'T') {
			current_player += 1;
		}

		if(current_time >= 210 && loser == 0) {
			loser = (current_player % 8) + 1;
		}
	}

	cout << loser << endl;

	return 0;
}
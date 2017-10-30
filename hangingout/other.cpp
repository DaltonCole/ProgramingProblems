#include <iostream>

using namespace std;

int main() {
	int max_people, cases, count, number, current_count;
	string direction;

	// People we turn away
	count = 0;
	// Current number of people in room
	current_count = 0;

	// max people, number of groups
	cin >> max_people >> cases;

	// For 
	for(int i = 0; i < cases; i++) {
		cin >> direction >> number;

		if(direction == "enter") {
			if(current_count + number <= max_people) {
				current_count += number;
			} else {
				count++;
			}
		} else {
			current_count -= number;
		}
	}
	cout << count << endl;

	return 0;
}
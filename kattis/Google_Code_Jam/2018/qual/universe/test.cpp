#include <iostream>
#include <vector>
#include <cmath>
using namespace std;

bool too_much_damage(const vector<int>& b, const int max_damage) {
	long current_damage = 0;

	for(int i = 0; i < b.size(); i++) {
		current_damage += (b[i] * pow(2, i));
		if(current_damage > max_damage) {
			return true;
		}
	}
	return false;
}

// For small input
int main() {
	int cases;

	cin >> cases;

	int damage;
	string shot;
	int total_shots;
	int last_index_with_elements;
	int swaps;

	vector<int> buckets;

	for(int i = 0; i < cases; i++) {
		cin >> damage >> shot;

		cout << "Case #" << (i + 1) << ": ";

		buckets.clear();
		buckets.push_back(0);
		total_shots = 0;
		last_index_with_elements = 0;
		swaps = 0;
		for(int j = 0; j < shot.length(); j++) {
			if(shot[j] == 'C') {
				buckets.push_back(0);
			} else {
				buckets[buckets.size() - 1]++;
				total_shots++;
				last_index_with_elements = buckets.size() - 1;
			}
		}

		// Impossible check
		if(total_shots > damage) {
			cout << "IMPOSSIBLE";
		} else { // It is possible
			while(too_much_damage(buckets, damage)) {
				swaps++;
				// Move last shot to previous bucket
				buckets[last_index_with_elements]--;
				buckets[last_index_with_elements - 1]++;
				// If bucket is now empty, move pointer to previous bucket
				if(buckets[last_index_with_elements] == 0) {
					last_index_with_elements--;
				}
			}
			cout << swaps;
		}

		cout << endl;
	}

	return 0;
}
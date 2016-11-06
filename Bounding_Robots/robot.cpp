#include <iostream>
using namespace std;

int main() {
	int l, w, cases;
	int thinks_l = 0, thinks_w = 0;
	int actual_l = 0, actual_w = 0;
	char direction;
	int steps;

	cin >> w >> l;

	while(w != 0 && l != 0) {
		cin >> cases;

		for(int i = 0; i < cases; i++) {
			cin >> direction >> steps;

			if(direction == 'u') {
				thinks_l += steps;
				actual_l += steps;
				if(actual_l >= l) {
					actual_l = l - 1;
				}
			} else if(direction == 'd') {
				thinks_l -= steps;
				actual_l -= steps;
				if(actual_l < 0) {
					actual_l = 0;
				}
			} else if(direction == 'l') {
				thinks_w -= steps;
				actual_w -= steps;
				if(actual_w < 0) {
					actual_w = 0;
				}
			} else if(direction == 'r') {
				thinks_w += steps;
				actual_w += steps;
				if(actual_w >= w) {
					actual_w = w - 1;
				}
			}
		}

		cout << "Robot thinks " << thinks_w << " " << thinks_l << endl;
		cout << "Actually at " << actual_w << " " << actual_l << endl << endl;

		thinks_w = 0;
		thinks_l = 0;
		actual_l = 0;
		actual_w = 0;

		cin >> w >> l;
	}

	return 0;
}
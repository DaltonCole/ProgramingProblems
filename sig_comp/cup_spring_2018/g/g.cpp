#include <iostream>
#include <vector>
#include <cmath>

using namespace std;

int reverse(int x) {
	int num = 0;
	int div = 1;

	while(x > 0) {
		num = num * 10;
		num += (x % 10);
		x /= 10;
	}

	return num;
}

int main() {
	vector<int> gen;

	for(int i = 100; i < 1000; i++) {
		gen.push_back(i * 1000 + reverse(i));
	}

	
	int cases;
	int num;
	int min_diff;
	int current_diff;
	int best;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> num;
		min_diff = 999999;

		for(int j = 0; j < gen.size(); j++) {
			current_diff = abs(gen[j] - num);

			if(current_diff < min_diff) {
				best = gen[j];
				min_diff = current_diff;
			}
		}

		cout << best << endl;
	}

	return 0;
}
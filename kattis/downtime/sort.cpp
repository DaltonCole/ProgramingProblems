include <iostream>
#include <algorithm>
#include <cmath>

using namespace std;

#define N 100000

int numbers[N];

int main() {
	int requests;
	int per_second;
	int temp;
	int current_window = 0;
	int max_window = 0;
	int smallest_index = 0;

	cin >> requests >> per_second;

	for(int i = 0; i < requests; i++) {
		cin >> temp;
		numbers[i] = temp;
	}

	for(int i = 0; i < requests; i++) {
		 if(numbers[smallest_index] + 1000 <= numbers[i]) {
		 	if(max_window < current_window) {
		 		max_window = current_window;
		 	}
		 	while(numbers[smallest_index] + 1000 <= numbers[i]) {
		 		smallest_index++;
		 		current_window--;
		 	}
		 	current_window++;
		 } else {
		 	current_window++;
		 }
	}

	if(max_window < current_window) {
		max_window = current_window;
	}

	cout << ceil(float(max_window) / float(per_second)) << endl;

	return 0;
}
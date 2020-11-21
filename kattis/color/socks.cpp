#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

struct Bounds {
	int count;
	int min;
};

int main() {
	int socks;
	int capacity;
	int max_difference;
	vector<int> colors;
	int temp;
	Bounds bucket;

	cin >> socks >> capacity >> max_difference;

	for(int i = 0; i < socks; i++) {
		cin >> temp;
		colors.push_back(temp);
	}

	stable_sort(colors.begin(), colors.end());
	int number = 0;

	for(auto i : colors) {
		if(bucket.count >= capacity || bucket.count == 0) {
			bucket.min = i;
			bucket.count = 1;
			number++;
		} else if(i - bucket.min <= max_difference) {
			bucket.count++;
		} else {
			bucket.min = i;
			bucket.count = 1;
			number++;
		}
	}

	cout << number << endl;

	return 0;
}
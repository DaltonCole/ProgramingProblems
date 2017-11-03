#include <iostream>
using namespace std;

int main() {
	int h, w, n;
	int sum;
	int temp;
	bool good = true;
	int current_height = 0;

	cin >> h >> w >> n;

	for(int i = 0; i < n; i++) {
		cin >> temp;
		sum += temp;
		if(sum >= w) {
			sum %= w;
			current_height++;
			if(sum != 0) {
				good = false;
				break;
			}
			if(current_height == h) {
				break;
			}
		}
	}
	if(current_height != h) {
		good = false;
	}

	if(good) {
		cout << "YES" << endl;
	} else {
		cout << "NO" << endl;
	}
	return 0;
}
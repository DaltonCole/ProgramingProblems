#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
	int vertices;
	int input;
	int count;
	vector<int> points;

	while(true) {
		cin >> vertices;
		if(vertices == -1) {
			break;
		}

		points.clear();

		for(int i = 0; i < vertices; i++) {
			count = 0;
			for(int j = 0; j < vertices; j++) {
				cin >> input;
				if(input == 1) {
					count++;
				}
			}
			if(count < 3) {
				points.push_back(i);
			}
		}
		sort(points.begin(), points.end());

		for(auto i : points) {
			cout << i << " ";
		}
		cout << endl;
	}

	return 0;
}
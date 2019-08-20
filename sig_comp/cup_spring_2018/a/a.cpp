#include <iostream>
#include <vector>

using namespace std;

int main() {
	int length;
	int known;
	int temp;

	vector<int> order;

	cin >> length >> known;
	for(int i = 0; i < known; i++) {
		cin >> temp;

		order.push_back(temp);
	}
}
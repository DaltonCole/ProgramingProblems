#include <iostream>
#include <vector>
using namespace std;

int main() {
	int cases;
	int max_number = 0;
	int current_number;
	vector<int> inputs;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> current_number;
		if(current_number > max_number) {
			max_number = current_number;
			inputs.push_back(max_number);
		}
	}
	cout << inputs.size() << endl;

	for(auto i : inputs) {
		cout << i << " ";
	}

	cout << endl;
}
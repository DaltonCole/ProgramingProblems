#include <iostream>
#include <vector>
using namespace std;



int main() {
	string stones;
	char current_stone;
	int count1 = 0, count2 = 0;
	vector<int> numbers;

	cin >> stones;

	if(stones.length() % 2 != 0) {
		cout << "0" << endl;
		return 0;
	} 

	for(int i = 0; i < stones.length() / 2; i++) {
		if(stones[i] == stones[stones.length() - i - 1]) {
			cout << "0" << endl;
			return 0;
		}
	}
	
	cout << 1 << endl;

	return 0;
}
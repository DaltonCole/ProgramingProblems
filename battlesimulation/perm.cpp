#include <iostream>
using namespace std;

int main() {
	string line;
	string sub = "";

	cin >> line;

	for(int i = 0; i < line.size(); i++) {
		sub += line[i];
		if(sub.size() > 3) {
			sub.erase(sub.begin());
		}

		if(sub == "RBL" || sub == "RLB" || sub == "BRL" || sub == "BLR" || sub == "LRB" || sub == "LBR") {
			line[i] = '-';
			line[i-1] = '-';
			line[i-2] = '-';
			sub = "";
		}
	}

	for(int i = 0; i < line.size(); i++) {
		if(line[i] == 'R') {
			cout << 'S';
		} else if(line[i] == 'B') {
			cout << 'K';
		} else if(line[i] == 'L') {
			cout << 'H';
		} else{
			cout << 'C';
			i += 2;
		}
	}
	cout << endl;

	return 0;
}
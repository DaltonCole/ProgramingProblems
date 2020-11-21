#include <iostream>
#include <string>

using namespace std;

int main() {
	string base;
	string full;
	int position = 0;

	while(getline(cin, base)) {
		getline(cin, full);
		
		position = 0;
		while(true) {
			position = full.find(base, position);

			if(position == -1) {
				break;
			} else {
				cout << position << " ";
				position++;
			}
		}
		cout << endl;
	}

	return 0;
}